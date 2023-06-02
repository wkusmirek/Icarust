//! Implement the acquistion service. Provides three `Acquisition` methods
//!
//! 1. watch_current_acquisition_run
//!     Streams AcquisitionRunInfo back to the server
//!
//! 2. current_status
//!     Gets the current status response (Unary)
//!
//! 3. get progress
//!     Returns a current progress response (Unary)
//!

use crate::services::minknow_api::acquisition::acquisition_service_server::AcquisitionService;
use crate::services::minknow_api::acquisition::get_progress_response::RawPerChannel;
use crate::services::minknow_api::acquisition::{
    AcquisitionRunInfo, CurrentStatusRequest, CurrentStatusResponse,
    GetCurrentAcquisitionRunRequest, GetProgressRequest, GetProgressResponse,
    WatchCurrentAcquisitionRunRequest, AcquisitionYieldSummary
};
use tokio::sync::mpsc;
use tokio_stream::wrappers::ReceiverStream;
use tonic::{Request, Response, Status};
use rand::Rng;

#[derive(Debug)]
pub struct Acquisition {
    pub run_id: String,
}

static mut read_count: i64 = 0;
static mut fraction_basecalled: f32 = 0.0;
static mut fraction_skipped: f32 = 0.0;
static mut basecalled_pass_read_count: i64 = 0;
static mut basecalled_fail_read_count: i64 = 0;
static mut basecalled_skipped_read_count: i64 = 0;
static mut basecalled_pass_bases: i64 = 0;
static mut basecalled_fail_bases: i64 = 0;
static mut basecalled_samples: i64 = 0;
static mut selected_raw_samples: i64 = 0;
static mut selected_events: i64 = 0;
static mut estimated_selected_bases: i64 = 0;
static mut alignment_matches: i64 = 0;
static mut alignment_mismatches: i64 = 0;
static mut alignment_insertions: i64 = 0;
static mut alignment_deletions: i64 = 0;
static mut alignment_coverage: f32 = 0.0;

#[tonic::async_trait]
impl AcquisitionService for Acquisition {
    type watch_current_acquisition_runStream = ReceiverStream<Result<AcquisitionRunInfo, Status>>;
    async fn watch_current_acquisition_run(
        &self,
        _request: Request<WatchCurrentAcquisitionRunRequest>,
    ) -> Result<Response<Self::watch_current_acquisition_runStream>, Status> {
        let (tx, rx) = mpsc::channel(4);
        let acquisition_run_info = AcquisitionRunInfo {
            run_id: "Wowee".to_string(),
            startup_state: 0,
            startup_state_estimated_end: None,
            startup_state_estimated_percent_complete: 0.0,
            state: 0,
            finishing_state: 0,
            stop_reason: 0,
            start_time: None,
            data_read_start_time: None,
            data_read_end_time: None,
            end_time: None,
            yield_summary: None,
            config_summary: None,
            writer_summary: None,
            bream_info: None,
        };
        tokio::spawn(async move {
            tx.send(Ok(acquisition_run_info.clone())).await.unwrap();
        });
        Ok(Response::new(ReceiverStream::new(rx)))
    }

    async fn get_current_acquisition_run(
        &self,
        _request: Request<GetCurrentAcquisitionRunRequest>,
    ) -> Result<Response<AcquisitionRunInfo>, Status> {
        unsafe {
        let num = rand::thread_rng().gen_range(0..100);
        if (num > 90) {read_count += 1;};
        if (num > 90) {fraction_basecalled = num as f32/101.0;};
        if (num > 90) {fraction_skipped = (101.0-num as f32)/101.0;};
        if (num >= 93) {basecalled_pass_read_count += 1;};
        if (num == 91) {basecalled_fail_read_count += 1;};
        if (num == 92) {basecalled_skipped_read_count += 2;};
        if (num >= 93) {basecalled_pass_bases += num*101;};
        if (num == 91) {basecalled_fail_bases = num*101;};
        basecalled_samples = 1;
        selected_raw_samples = 1;
        if (num > 90) {selected_events += num*101;};
        if (num > 90) {estimated_selected_bases += num*101;};
        if (num > 94) {alignment_matches += num*101;};
        if (num == 91) {alignment_mismatches += num*11;};
        if (num == 92) {alignment_insertions += num*11;};
        if (num == 93) {alignment_deletions += num*11;};
        if (num > 90 && alignment_coverage < 0.9) {alignment_coverage += 0.05;};
        let yieldSummary = AcquisitionYieldSummary {
            read_count: read_count,
            fraction_basecalled: fraction_basecalled,
            fraction_skipped: fraction_skipped,
            basecalled_pass_read_count: basecalled_pass_read_count,
            basecalled_fail_read_count: basecalled_fail_read_count,
            basecalled_skipped_read_count: basecalled_skipped_read_count,
            basecalled_pass_bases: basecalled_pass_bases,
            basecalled_fail_bases: basecalled_fail_bases,
            basecalled_samples: basecalled_samples,
            selected_raw_samples: selected_raw_samples,
            selected_events: selected_events,
            estimated_selected_bases: estimated_selected_bases,
            alignment_matches: alignment_matches,
            alignment_mismatches: alignment_mismatches,
            alignment_insertions: alignment_insertions,
            alignment_deletions: alignment_deletions,
            alignment_coverage: alignment_coverage,
        };
        Ok(Response::new(AcquisitionRunInfo {
            run_id: self.run_id.clone(),
            startup_state: 0,
            startup_state_estimated_end: None,
            startup_state_estimated_percent_complete: 0.0,
            state: 0,
            finishing_state: 0,
            stop_reason: 0,
            start_time: None,
            data_read_start_time: None,
            data_read_end_time: None,
            end_time: None,
            yield_summary: Some(yieldSummary),
            config_summary: None,
            writer_summary: None,
            bream_info: None,
        }))
        }
    }

    async fn current_status(
        &self,
        _request: Request<CurrentStatusRequest>,
    ) -> Result<Response<CurrentStatusResponse>, Status> {
        Ok(Response::new(CurrentStatusResponse { status: 3 }))
    }

    async fn get_progress(
        &self,
        _request: Request<GetProgressRequest>,
    ) -> Result<Response<GetProgressResponse>, Status> {
        Ok(Response::new(GetProgressResponse {
            raw_per_channel: Some(RawPerChannel {
                acquired: 100,
                processed: 900,
            }),
        }))
    }
}
