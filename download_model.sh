#!/bin/bash
mkdir -p model
cd model
curl -L -o model.zip https://alphacephei.com/vosk/models/vosk-model-small-en-us-0.15.zip
unzip model.zip
mv vosk-model-small-en-us-0.15/* .
rm -r vosk-model-small-en-us-0.15
rm model.zip
