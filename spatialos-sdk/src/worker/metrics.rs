use std::ffi::CString;
use std::slice;

use crate::worker::internal::utils::cstr_to_string;
use spatialos_sdk_sys::worker::{
    Worker_GaugeMetric, Worker_HistogramMetric, Worker_HistogramMetricBucket, Worker_Metrics,
};

pub struct Metrics {
    pub load: Option<f64>,
    pub gauge_metrics: Vec<GaugeMetric>,
    pub histogram_metrics: Vec<HistogramMetric>,
}

impl Metrics {
    pub(crate) fn to_worker_sdk(&self) -> WrappedWorkerMetrics {
        // Pre-allocate the storage.
        let mut keys: Vec<CString> =
            Vec::with_capacity(self.gauge_metrics.len() + self.histogram_metrics.len());
        let mut gauge_metrics: Vec<Worker_GaugeMetric> =
            Vec::with_capacity(self.gauge_metrics.len());
        let mut histogram_metrics: Vec<Worker_HistogramMetric> =
            Vec::with_capacity(self.histogram_metrics.len());
        let num_buckets = self
            .histogram_metrics
            .iter()
            .map(|m| m.buckets.len())
            .sum::<usize>();
        let mut histogram_metrics_buckets: Vec<Worker_HistogramMetricBucket> =
            Vec::with_capacity(num_buckets);

        self.gauge_metrics
            .iter()
            .map(|m| m.to_worker_sdk(&mut keys))
            .for_each(|m| gauge_metrics.push(m));

        self.histogram_metrics
            .iter()
            .map(|m| m.to_worker_sdk(&mut keys, &mut histogram_metrics_buckets))
            .for_each(|m| histogram_metrics.push(m));

        WrappedWorkerMetrics {
            metrics: Worker_Metrics {
                load: match self.load {
                    Some(c) => &c,
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
    fn from(metrics: &Worker_Metrics) -> Self {
        let gauge_metrics =
            unsafe { slice::from_raw_parts(metrics.gauge_metrics, metrics.gauge_metric_count as usize)
                .iter()
                .map(|m| GaugeMetric::from(m))
                .collect() };

        let histogram_metrics = unsafe { slice::from_raw_parts(
            metrics.histogram_metrics,
            metrics.histogram_metric_count as usize,
        )
        .iter()
        .map(|m| HistogramMetric::from(m))
        .collect() };

        let load = if metrics.load.is_null() {
            None
        } else {
            unsafe { Some(*metrics.load) }
        };

        Metrics {
            load,
            gauge_metrics,
            histogram_metrics,
        }
    }
}

pub struct GaugeMetric {
    pub key: String,
    pub value: f64,
}

impl GaugeMetric {
    pub(crate) fn to_worker_sdk(&self, cstrs: &mut Vec<CString>) -> Worker_GaugeMetric {
        let cstr = CString::new(self.key.as_str()).unwrap();
        cstrs.push(cstr);

        Worker_GaugeMetric {
            key: cstrs.last().unwrap().as_ptr(),
            value: self.value,
        }
    }
}

impl From<&Worker_GaugeMetric> for GaugeMetric {
    fn from(gauge_metric: &Worker_GaugeMetric) -> Self {
        GaugeMetric {
            key: cstr_to_string(gauge_metric.key),
            value: gauge_metric.value,
        }
    }
}

pub struct HistogramMetric {
    pub key: String,
    pub sum: f64,
    pub buckets: Vec<HistogramMetricBucket>,
}

impl HistogramMetric {
    pub(crate) fn to_worker_sdk(
        &self,
        keys: &mut Vec<CString>,
        buckets: &mut Vec<Worker_HistogramMetricBucket>,
    ) -> Worker_HistogramMetric {
        let cstr = CString::new(self.key.as_str()).unwrap();
        keys.push(cstr);

        let first_element_index = buckets.len();
        self.buckets
            .iter()
            .map(|b| Worker_HistogramMetricBucket {
                upper_bound: b.upper_bound,
                samples: b.samples,
            })
            .for_each(|b| buckets.push(b));

        Worker_HistogramMetric {
            key: keys.last().unwrap().as_ptr(),
            bucket_count: self.buckets.len() as u32,
            buckets: &buckets[first_element_index],
            sum: self.sum,
        }
    }
}

impl From<&Worker_HistogramMetric> for HistogramMetric {
    fn from(histogram_metric: &Worker_HistogramMetric) -> Self {
        let buckets = unsafe { slice::from_raw_parts(
            histogram_metric.buckets,
            histogram_metric.bucket_count as usize,
        )
        .iter()
        .map(|bucket| HistogramMetricBucket {
            upper_bound: bucket.upper_bound,
            samples: bucket.samples,
        })
        .collect() };

        HistogramMetric {
            key: cstr_to_string(histogram_metric.key),
            sum: histogram_metric.sum,
            buckets,
        }
    }
}

pub struct HistogramMetricBucket {
    pub upper_bound: f64,
    pub samples: u32,
}
