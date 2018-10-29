use worker::internal::bindings::{
    Worker_GaugeMetric, Worker_HistogramMetric, Worker_HistogramMetricBucket, Worker_Metrics,
};
use worker::internal::utils::cstr_to_string;

pub struct Metrics {
    pub load: f64,
    pub gauge_metrics: Vec<GaugeMetric>,
    pub histogram_metrics: Vec<HistogramMetric>,
}

impl Metrics {
    pub(crate) fn from_worker_sdk(metrics: &Worker_Metrics) -> Self {
        let mut gauge_metrics = Vec::new();
        let mut histogram_metrics = Vec::new();

        unsafe {
            for i in 0..metrics.gauge_metric_count as isize {
                let ptr = metrics.gauge_metrics.offset(i) as *const Worker_GaugeMetric;
                assert!(!ptr.is_null());
                gauge_metrics.push(GaugeMetric::from_worker_sdk(*ptr));
            }
        }

        unsafe {
            for i in 0..metrics.histogram_metric_count as isize {
                let ptr = metrics.histogram_metrics.offset(i) as *const Worker_HistogramMetric;
                assert!(!ptr.is_null());
                histogram_metrics.push(HistogramMetric::from_worker_sdk(*ptr));
            }
        }

        let load = unsafe { *metrics.load };

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
    fn from_worker_sdk(gauge_metric: Worker_GaugeMetric) -> Self {
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
    fn from_worker_sdk(histogram_metric: Worker_HistogramMetric) -> Self {
        let mut buckets = Vec::new();

        unsafe {
            for i in 0..histogram_metric.bucket_count as isize {
                let bucket_ptr =
                    histogram_metric.buckets.offset(i) as *const Worker_HistogramMetricBucket;
                assert!(!bucket_ptr.is_null());
                buckets.push(HistogramMetricBucket {
                    upper_bound: (*bucket_ptr).upper_bound,
                    samples: (*bucket_ptr).samples,
                });
            }
        }

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
