import { gql } from '@apollo/client';

// GraphQL mutations for vision operations
export const SAVE_RECOGNITION_RESULT = gql`
  mutation SaveRecognitionResult($input: RecognitionResultInput!) {
    saveRecognitionResult(input: $input) {
      id
      detections {
        label
        confidence
        bbox {
          x
          y
          width
          height
        }
      }
      processingTimeMs
      timestamp
    }
  }
`;

// GraphQL queries for retrieving recognition results
export const GET_RECOGNITION_RESULTS = gql`
  query GetRecognitionResults(
    $limit: Int = 10
    $offset: Int = 0
    $dateFrom: DateTime
    $dateTo: DateTime
  ) {
    recognitionResults(
      limit: $limit
      offset: $offset
      dateFrom: $dateFrom
      dateTo: $dateTo
    ) {
      id
      detections {
        label
        confidence
        bbox {
          x
          y
          width
          height
        }
      }
      processingTimeMs
      timestamp
      imageUrl
    }
  }
`;

// GraphQL subscription for real-time recognition updates
export const RECOGNITION_RESULT_SUBSCRIPTION = gql`
  subscription OnRecognitionResult {
    recognitionResultAdded {
      id
      detections {
        label
        confidence
        bbox {
          x
          y
          width
          height
        }
      }
      processingTimeMs
      timestamp
    }
  }
`;

// GraphQL mutation to delete recognition result
export const DELETE_RECOGNITION_RESULT = gql`
  mutation DeleteRecognitionResult($id: ID!) {
    deleteRecognitionResult(id: $id) {
      success
      message
    }
  }
`;

// GraphQL query for recognition statistics
export const GET_RECOGNITION_STATS = gql`
  query GetRecognitionStats($dateFrom: DateTime, $dateTo: DateTime) {
    recognitionStats(dateFrom: $dateFrom, dateTo: $dateTo) {
      totalRecognitions
      uniqueLabels
      averageConfidence
      averageProcessingTime
      topLabels {
        label
        count
      }
    }
  }
`;

// GraphQL mutation for bulk recognition
export const PROCESS_BATCH_IMAGES = gql`
  mutation ProcessBatchImages($images: [ImageInput!]!) {
    processBatchImages(images: $images) {
      id
      status
      results {
        detections {
          label
          confidence
          bbox {
            x
            y
            width
            height
          }
        }
        processingTimeMs
      }
    }
  }
`;

// Helper functions for vision operations
export const visionUtils = {
  // Format detection results for display
  formatDetections(detections) {
    return detections.map(detection => ({
      ...detection,
      displayLabel: detection.label.replace(/_/g, ' ').replace(/\b\w/g, l => l.toUpperCase()),
      displayConfidence: Math.round(detection.confidence * 100)
    }));
  },
  
  // Group detections by label
  groupByLabel(detections) {
    const groups = {};
    detections.forEach(detection => {
      if (!groups[detection.label]) {
        groups[detection.label] = [];
      }
      groups[detection.label].push(detection);
    });
    return groups;
  },
  
  // Filter detections by confidence threshold
  filterByConfidence(detections, threshold = 0.5) {
    return detections.filter(d => d.confidence >= threshold);
  },
  
  // Calculate average confidence
  calculateAverageConfidence(detections) {
    if (!detections.length) return 0;
    const sum = detections.reduce((acc, d) => acc + d.confidence, 0);
    return sum / detections.length;
  }
};

// GraphQL fragments for common vision types
export const VISION_FRAGMENTS = {
  detection: gql`
    fragment DetectionFields on Detection {
      label
      confidence
      bbox {
        x
        y
        width
        height
      }
    }
  `,
  
  recognitionResult: gql`
    fragment RecognitionResultFields on RecognitionResult {
      id
      detections {
        ...DetectionFields
      }
      processingTimeMs
      timestamp
    }
  `
};