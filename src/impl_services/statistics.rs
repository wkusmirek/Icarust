//pub(crate) mod statistics{
//    tonic::include_proto!("minknow_api.statistics");
//}

use rand::Rng;
use crate::services::minknow_api::statistics;
use crate::services::minknow_api::statistics::statistics_service_server::StatisticsService;
use crate::services::minknow_api::statistics::stream_read_length_histogram_response::BucketRange;
use crate::services::minknow_api::statistics::stream_read_length_histogram_response::ReadLengthHistogramData;
use tonic::{Request, Response, Status};
use tokio::sync::mpsc;
use tokio_stream::wrappers::ReceiverStream;

#[derive(Debug)]
pub struct Statistics {
    //channel_size: usize
}

impl Statistics {
    pub fn new (channel_size: usize) -> Statistics {
        Statistics {
            //channel_size
        }
    }
}

#[tonic::async_trait]
impl StatisticsService for Statistics {

    type stream_read_length_histogramStream = ReceiverStream<Result<statistics::StreamReadLengthHistogramResponse, Status>>;

    async fn stream_read_length_histogram(
        &self,
        _request: Request<statistics::StreamReadLengthHistogramRequest>,
    ) -> Result<Response<Self::stream_read_length_histogramStream>, Status> {
        let mut rng = rand::thread_rng();
        let mut bucket_ranges: Vec<BucketRange> = vec![];
        bucket_ranges.push(BucketRange{start:0, end:128});
        bucket_ranges.push(BucketRange{start:128, end:256});
        bucket_ranges.push(BucketRange{start:256, end:354});
        let mut histogram_data: Vec<ReadLengthHistogramData> = vec![];
        //histogram_data.push(ReadLengthHistogramData{});
        let (tx, rx) = mpsc::channel(4);
        let positions = statistics::StreamReadLengthHistogramResponse {
            read_length_type: 1, // ReadLengthType::MinknowEvents
            bucket_ranges: bucket_ranges,
            source_data_end: 100,
            bucket_value_type: 2, // BucketValueType::ReadLengths
            histogram_data: histogram_data,
        };
        tokio::spawn(async move {
            tx.send(Ok(positions.clone())).await.unwrap();
        });
        Ok(Response::new(ReceiverStream::new(rx)))
    }
    
}




/*

message StreamReadLengthHistogramResponse {
    // The data source for the histograms
    //
    // Also specifies the units for `data_selection` and `n50`
    //
    // See `ReadLengthType` for further information about the possible options.
    //
    ReadLengthType read_length_type = 1;

    message BucketRange {
        uint64 start = 1;
        uint64 end = 2;
    }

    // The range covered by each bucket in the histogram data
    repeated BucketRange bucket_ranges = 2;

    // The right hand edge of the last source bucket which contains data
    //
    // Measured across all source data, after excluding the reads specified by
    // `discard_outlier_percent` in the stream request.
    //
    uint64 source_data_end = 5;

    // The data accumulated in the read length histogram buckets
    //
    // See `BucketValueType` for further information about the possible options.
    //
    BucketValueType bucket_value_type = 3;

    message ReadLengthHistogramData {
        // The filtering parameters which contributed to this bucket.
        repeated ReadLengthHistogramKey filtering = 3;

        // Counts for each histogram bucket
        //
        // Units are as specified in `read_length_type`
        // The range covered by each bucket is as in `bucket_ranges`
        // The type of data accumulated in each bucket is given by `bucket_value_type`
        //
        repeated uint64 bucket_values = 1;

        // The N50 value for the read length data for the selected `read_length_type` and
        // `read_end_reasons`.
        //
        // Units are as specified by `read_length_type`.
        //
        // Measured across all source data, after excluding the reads specified by
        // `discard_outlier_percent` in the stream request.
        //
        float n50 = 2;
    }

    // The histogram data
    repeated ReadLengthHistogramData histogram_data = 4;
}

*/
