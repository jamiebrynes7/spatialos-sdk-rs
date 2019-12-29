use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::ffi::CString;
use std::slice;

use crate::worker::utils::cstr_to_string;
use spatialos_sdk_sys::worker::{
    Worker_GaugeMetric, Worker_HistogramMetric, Worker_HistogramMetricBucket, Worker_Metrics,
};

#[derive(Debug)]
pub enum MetricsError {
    KeyAlreadyExists,
}

#[derive(Debug, Default)]
pub struct Metrics {
    pub load: Option<f64>,
    pub gauge_metrics: HashMap<String, f64>,
    pub histogram_metrics: HashMap<String, HistogramMetric>,
}

impl Metrics {
    pub fn new() -> Self {
        Metrics {
            load: None,
            gauge_metrics: HashMap::new(),
            histogram_metrics: HashMap::new(),
        }
    }

    pub fn with_load(mut self, load: f64) -> Self {
        self.load = Some(load);
        self
    }

    pub fn with_histogram_metric<T: Into<String>>(
        mut self,
        key: T,
        histogram_metric: HistogramMetric,
    ) -> Self {
        self.histogram_metrics.insert(key.into(), histogram_metric);
        self
    }

    pub fn with_gauge_metric<T: Into<String>>(mut self, key: T, value: f64) -> Self {
        self.gauge_metrics.insert(key.into(), value);
        self
    }

    pub fn add_gauge_metric<T: Into<String>>(&mut self, key: T) -> Result<&mut f64, MetricsError> {
        match self.gauge_metrics.entry(key.into()) {
            Entry::Occupied(_) => Err(MetricsError::KeyAlreadyExists),
            Entry::Vacant(entry) => Ok(entry.insert(0.0)),
        }
    }

    pub fn add_histogram_metric<T: Into<String>>(
        &mut self,
        key: T,
        bounds: &[f64],
    ) -> Result<&mut HistogramMetric, MetricsError> {
        match self.histogram_metrics.entry(key.into()) {
            Entry::Occupied(_) => Err(MetricsError::KeyAlreadyExists),
            Entry::Vacant(entry) => Ok(entry.insert(HistogramMetric::new(bounds))),
        }
    }

    pub(crate) fn to_worker_sdk(&self) -> WrappedWorkerMetrics {
        // Pre-allocate the storage.
        let mut keys: Vec<CString> =
            Vec::with_capacity(self.gauge_metrics.len() + self.histogram_metrics.len());

        let gauge_metrics = self
            .gauge_metrics
            .iter()
            .map(|m| {
                let c_str = CString::new(m.0.as_str()).unwrap();
                keys.push(c_str);
                Worker_GaugeMetric {
                    key: keys.last().unwrap().as_ptr(),
                    value: *m.1,
                }
            })
            .collect::<Vec<Worker_GaugeMetric>>();

        let num_buckets = self
            .histogram_metrics
            .iter()
            .map(|m| m.1.buckets.len())
            .sum::<usize>();
        let mut histogram_metrics_buckets: Vec<Worker_HistogramMetricBucket> =
            Vec::with_capacity(num_buckets);

        let histogram_metrics = self
            .histogram_metrics
            .iter()
            .map(|m| {
                let c_str = CString::new(m.0.as_str()).unwrap();
                keys.push(c_str);
                m.1.to_worker_sdk(
                    keys.last().unwrap().as_ptr(),
                    &mut histogram_metrics_buckets,
                )
            })
            .collect::<Vec<Worker_HistogramMetric>>();

        WrappedWorkerMetrics {
            metrics: Worker_Metrics {
                load: match self.load {
                    Some(ref c) => c,
                    None => ::std::ptr::null(),
                },
                gauge_metric_count: gauge_metrics.len() as u32,
                gauge_metrics: gauge_metrics.as_ptr(),
                histogram_metric_count: histogram_metrics.len() as u32,
                histogram_metrics: histogram_metrics.as_ptr(),
            },
            _gauge_metrics: gauge_metrics,
            _histogram_metrics: histogram_metrics,
            _histogram_metrics_buckets: histogram_metrics_buckets,
            _keys: keys,
        }
    }
}

pub(crate) struct WrappedWorkerMetrics {
    pub metrics: Worker_Metrics,
    _gauge_metrics: Vec<Worker_GaugeMetric>,
    _histogram_metrics: Vec<Worker_HistogramMetric>,
    _histogram_metrics_buckets: Vec<Worker_HistogramMetricBucket>,
    _keys: Vec<CString>,
}

impl From<&Worker_Metrics> for Metrics {
    fn from(worker_metrics: &Worker_Metrics) -> Self {
        let mut metrics = Metrics::new();

        unsafe {
            slice::from_raw_parts(
                worker_metrics.gauge_metrics,
                worker_metrics.gauge_metric_count as usize,
            )
            .iter()
            .for_each(|gauge_metric| {
                metrics
                    .gauge_metrics
                    .insert(cstr_to_string(gauge_metric.key), gauge_metric.value);
            });
        }

        unsafe {
            slice::from_raw_parts(
                worker_metrics.histogram_metrics,
                worker_metrics.histogram_metric_count as usize,
            )
            .iter()
            .for_each(|histogram_metric| {
                metrics.histogram_metrics.insert(
                    cstr_to_string(histogram_metric.key),
                    HistogramMetric::from(histogram_metric),
                );
            });
        }

        if !worker_metrics.load.is_null() {
            metrics.load = Some(unsafe { *worker_metrics.load });
        }

        metrics
    }
}

#[derive(Debug)]
pub struct HistogramMetric {
    pub sum: f64,
    pub buckets: Vec<HistogramMetricBucket>,
}

impl HistogramMetric {
    pub fn new(bounds: &[f64]) -> Self {
        let mut buckets = bounds
            .iter()
            .map(|bound| HistogramMetricBucket::new(*bound))
            .collect::<Vec<HistogramMetricBucket>>();

        buckets.sort_by(|bucket1, bucket2| {
            bucket1
                .upper_bound
                .partial_cmp(&bucket2.upper_bound)
                .unwrap()
        });

        buckets.push(HistogramMetricBucket::new(::std::f64::MAX));

        HistogramMetric { sum: 0.0, buckets }
    }

    pub fn add_sample(&mut self, value: f64) {
        self.buckets
            .iter_mut()
            .filter(|bucket| bucket.upper_bound >= value)
            .for_each(|bucket| bucket.samples += 1);

        self.sum += value;
    }

    pub fn reset(&mut self) {
        self.sum = 0.0;

        self.buckets
            .iter_mut()
            .for_each(|bucket| bucket.samples = 0);
    }

    pub(crate) fn to_worker_sdk(
        &self,
        key: *const ::std::os::raw::c_char,
        buckets: &mut Vec<Worker_HistogramMetricBucket>,
    ) -> Worker_HistogramMetric {
        let first_element_index = buckets.len();
        self.buckets
            .iter()
            .map(|b| Worker_HistogramMetricBucket {
                upper_bound: b.upper_bound,
                samples: b.samples,
            })
            .for_each(|b| buckets.push(b));

        Worker_HistogramMetric {
            key,
            bucket_count: self.buckets.len() as u32,
            buckets: &buckets[first_element_index],
            sum: self.sum,
        }
    }
}

impl From<&Worker_HistogramMetric> for HistogramMetric {
    fn from(histogram_metric: &Worker_HistogramMetric) -> Self {
        let buckets = unsafe {
            slice::from_raw_parts(
                histogram_metric.buckets,
                histogram_metric.bucket_count as usize,
            )
            .iter()
            .map(|bucket| HistogramMetricBucket {
                upper_bound: bucket.upper_bound,
                samples: bucket.samples,
            })
            .collect()
        };

        HistogramMetric {
            sum: histogram_metric.sum,
            buckets,
        }
    }
}

#[derive(Debug)]
pub struct HistogramMetricBucket {
    pub upper_bound: f64,
    pub samples: u32,
}

impl HistogramMetricBucket {
    pub fn new(upper_bound: f64) -> Self {
        HistogramMetricBucket {
            upper_bound,
            samples: 0,
        }
    }
}

#[cfg(test)]
mod test {
    use crate::worker::metrics::HistogramMetric;

    #[test]
    pub fn histogram_metric_bounds_are_sorted() {
        let metric = HistogramMetric::new(&[10.0, 5.0, 15.0, 3.0]);

        let (success, _) = metric
            .buckets
            .iter()
            .map(|m| m.upper_bound)
            .fold((true, ::std::f64::MIN), |last, next| (last.1 <= next, next));

        assert_eq!(true, success);
    }

    #[test]
    pub fn histogram_metric_add_sample() {
        let mut metric = HistogramMetric::new(&[10.0, 5.0, 15.0, 3.0]);

        // Test halfway through.
        metric.add_sample(7.5);
        assert_eq!(0, metric.buckets[0].samples);
        assert_eq!(0, metric.buckets[1].samples);
        assert_eq!(1, metric.buckets[2].samples);
        assert_eq!(1, metric.buckets[3].samples);

        metric.reset();

        // Test everything.
        metric.add_sample(1.0);
        assert_eq!(1, metric.buckets[0].samples);
        assert_eq!(1, metric.buckets[1].samples);
        assert_eq!(1, metric.buckets[2].samples);
        assert_eq!(1, metric.buckets[3].samples);
    }
}
