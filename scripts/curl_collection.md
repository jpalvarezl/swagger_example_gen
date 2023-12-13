# Setup

## Needed dependencies

- `curl`
- `jq`

## Tips

- For streaming scenarios `jq` doesn't work, since it's not technically JSON that comes back (as a single response). In that case is better to dump into a file by adding ` > dump.json` after the `curl` call
- Be mindful of trailing space after `\` (that took me too much time to figure out)

## Curl template

```bash
curl -s -X POST "${AZURE_OPENAI_ENDPOINT}/${PATH_TO_METHOD}?api-version=2023-12-01-preview" \
-H "api-key: ${AZURE_OPENAI_KEY}" \
-H "accept: application/json" \
-d '{
   "your" : "json",
   "goes": "here"
}'
```

# Whisper

## transcriptions

```bash
curl --trace-ascii ./traces/audio_transcription.trace \
   -l "${AZURE_OPENAI_ENDPOINT}/openai/deployments/whisper-deployment/audio/transcriptions?api-version=2023-09-01-preview" \
   -H "api-key: ${AZURE_OPENAI_KEY}" \
   -H "Content-Type: multipart/form-data" \
   -F "file=@./sample_files/batman.wav"
```

## Transalation

```bash
curl --trace-ascii ./traces/audio_translation.trace \
   -l "${AZURE_OPENAI_ENDPOINT}/openai/deployments/whisper-deployment/audio/translations?api-version=2023-09-01-preview" \
   -H "api-key: ${AZURE_OPENAI_KEY}" \
   -H "Content-Type: multipart/form-data" \
   -F "file=@./sample_files/JP_it_is_rainy_today.wav"
```

# Translation: response as plain/text

``` bash
# adding field for a plain/text format response
curl --trace-ascii ./traces/audio_translation_text.trace \
   -l "${AZURE_OPENAI_ENDPOINT}/openai/deployments/whisper-deployment/audio/translations?api-version=2023-09-01-preview" \
   -H "api-key: ${AZURE_OPENAI_KEY}" \
   -H "Content-Type: multipart/form-data" \
   -F "file=@./sample_files/JP_it_is_rainy_today.wav" \
   -F "response_format=text"
```

# Chat Completions

Add `"stream": true` at the top level of the request JSON for streaming response

## Tool calls

```bash
curl -v -X POST "${AZURE_OPENAI_ENDPOINT}/openai/deployments/gpt-4-1106-preview/chat/completions?api-version=2023-12-01-preview" \
-H "api-key: ${AZURE_OPENAI_KEY}" \
-H "Content-Type: application/json" \
-d '{
      "messages": [
         {
         "role": "system",
         "content": "You are a helpful assistant."
         },
         {
         "role": "user",
         "content": "What should I wear in Honolulu next Thursday?"
         }
      ],
      "tools": [
         {
         "type": "function",
         "function": {
            "name": "FutureTemperature",
            "parameters": {
               "type": "object",
               "properties": {
               "date": {
                  "description": "The date of the weather forecast.",
                  "type": "string"
               },
               "locationName": {
                  "description": "The name of the location to forecast the weather for.",
                  "type": "string"
               },
               "unit": {
                  "description": "The unit of measurement for the temperature.",
                  "type": "string"
               }
               }
            }
         }
         }
      ]
   }'
```

## With images

```bash
curl -v -X POST "${OPENAI_ENDPOINT}/chat/completions" \
-H "Content-Type: application/json" \
-H "Authorization: Bearer ${OPENAI_KEY}" \
-d '{
   "stream": true,
   "messages": [
      {
      "role": "system",
      "content": "You are a helpful assistant that describes images"
      },
      {
      "role": "user",
      "content": [
         {
            "type": "text",
            "text": "Please describe this image"
         },
         {
            "type": "image_url",
            "image_url": {
            "url": "https://upload.wikimedia.org/wikipedia/commons/thumb/4/44/Microsoft_logo.svg/480px-Microsoft_logo.svg.png"
            }
         }
      ]
      }
   ],
   "max_tokens": 2048,
   "model": "gpt-4-vision-preview"
}'
```

# Legacy Completions
