 #!/bin/sh

 DETECTION_MODEL="https://ocrs-models.s3-accelerate.amazonaws.com/text-detection.rten"
 RECOGNITION_MODEL="https://ocrs-models.s3-accelerate.amazonaws.com/text-recognition.rten"

 curl "$DETECTION_MODEL" -o ../models/text-detection.rten
 curl "$RECOGNITION_MODEL" -o ../models/text-recognition.rten
