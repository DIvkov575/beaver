import apache_beam as beam
from apache_beam.options.pipeline_options import PipelineOptions


class PubSubToBigQueryOptions(PipelineOptions):
    @classmethod
    def _add_argparse_args(cls, parser):
        parser.add_argument('--input_topic', help='Input Pub/Sub topic')
        parser.add_argument('--output_table', help='Output BigQuery table')


def run():
    # Set the pipeline options
    pipeline_options = PipelineOptions()
    pubsub_to_bigquery_options = pipeline_options.view_as(PubSubToBigQueryOptions)

    with beam.Pipeline(options=pipeline_options) as p:
        # Read from Pub/Sub
        messages = (p
                    | 'ReadFromPubSub' >> beam.io.ReadFromPubSub(topic=pubsub_to_bigquery_options.input_topic)
                    | 'ParseJSON' >> beam.Map(lambda x: eval(x.decode('utf-8'))))

        # Write to BigQuery
        messages | 'WriteToBigQuery' >> beam.io.WriteToBigQuery(
            table=pubsub_to_bigquery_options.output_table,
            schema='field1:STRING,field2:INTEGER,field3:FLOAT',  # Update with your schema
            create_disposition=beam.io.BigQueryDisposition.CREATE_IF_NEEDED,
            write_disposition=beam.io.BigQueryDisposition.WRITE_APPEND)


if __name__ == '__main__':
    run()
