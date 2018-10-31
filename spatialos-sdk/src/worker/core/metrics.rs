use std::slice;

use worker::internal::bindings::{
    Worker_GaugeMetric, Worker_HistogramMetric, Worker_HistogramMetricBucket, Worker_Metrics,
};
use worker::internal::utils::cstr_to_string;
use worker::internal::worker_sdk_conversion::WorkerSdkConversion;

pub struct Metrics {
    pub load: Option<f64>,
    pub gauge_metrics: Vec<GaugeMetric>,
    pub histogram_metrics: Vec<HistogramMetric>,
}

unsafe impl WorkerSdkConversion<Worker_Metrics> for Metrics {
    unsafe fn from_worker_sdk(metrics: &Worker_Metrics) -> Self {
        let gauge_metrics =
            slice::from_raw_parts(metrics.gauge_metrics, metrics.gauge_metric_count as usize)
                .iter()
                .map(|m| GaugeMetric::from_worker_sdk(m))
                .collect();

        let histogram_metrics = slice::from_raw_parts(
            metrics.histogram_metrics,
            metrics.histogram_metric_count as usize,
        ).iter()
        .map(|m| HistogramMetric::from_worker_sdk(m))
        .collect();

        let load = match metrics.load.is_null() {
            true => None,
            false => Some(*metrics.load),
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

unsafe impl WorkerSdkConversion<Worker_GaugeMetric> for GaugeMetric {
    unsafe fn from_worker_sdk(gauge_metric: &Worker_GaugeMetric) -> Self {
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

unsafe impl WorkerSdkConversion<Worker_HistogramMetric> for HistogramMetric {
    unsafe fn from_worker_sdk(histogram_metric: &Worker_HistogramMetric) -> Self {
        let buckets = slice::from_raw_parts(
            histogram_metric.buckets,
            histogram_metric.bucket_count as usize,
        ).iter()
        .map(|bucket| HistogramMetricBucket {
            upper_bound: bucket.upper_bound,
            samples: bucket.samples,
        }).collect();

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
