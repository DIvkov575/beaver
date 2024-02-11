import logging

import apache_beam as beam
from apache_beam.options.pipeline_options import PipelineOptions


def process_batch(batch):
    processed_batch = []
    for element in batch:
        # Perform processing on each element
        logging.info("stuff")
        processed_batch.append(element)
    return processed_batch


def run(argv=None):
    # Define pipeline options
    options = PipelineOptions(argv)

    # Create a Pipeline with the specified options
    with beam.Pipeline(options=options) as p:
        input_data = (
                p
                | 'ReadFromPubSub' >> beam.io.ReadFromPubSub(subscription="projects/your-project/subscriptions/your-subscription")
                | 'ProcessBatch' >> beam.ParDo(process_batch)
        )


if __name__ == '__main__':
    logging.getLogger().setLevel(logging.INFO)
    run()
