pub unsafe trait WorkerSdkConversion<T> {
    unsafe fn from_worker_sdk(sdk_obj: &T) -> Self;
    //fn to_worker_sdk(&self) -> T;
}
