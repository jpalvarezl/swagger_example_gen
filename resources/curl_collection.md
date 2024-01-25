# Setup

## Needed dependencies

- `curl`
- `jq`

## Tips

- For streaming scenarios `jq` doesn't work, since it's not technically JSON that comes back (as a single response). In that case is better to dump into a file by adding ` > dump.json` after the `curl` call
- Be mindful of trailing space after `\` (that took me too much time to figure out)

## Curl template

Notice the authentication is different depending on whether we are using Azure or not.

### Non Azure OpenAI curl preamble

```bash
curl -v -X POST "${OPENAI_ENDPOINT}/chat/completions" \
-H "Authorization: Bearer ${OPENAI_KEY}" \
-d '{
   "your" : "json",
   "goes": "here"
}'
```

### Azure OpenAI preamble

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

### Azure

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

### Non Azure

```bash
curl -v -X POST "${OPENAI_ENDPOINT}/chat/completions" \
-H "Content-Type: application/json" \
-H "Authorization: Bearer ${OPENAI_KEY}" \
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
      "stream": true,
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
      ],
      "model": "gpt-3.5-turbo-1106"
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

## Notes

- Setting recording longer

- Spoon theory! It's the right name for it, just rebranded.

- Promo is not a side-effect. "Just do good work"

- I did get a perspective from Shawn.
- Nice to hear that people are doing the connects actually! Pinging directly sounds like a good strategy to get it :) 

- Keeping visibility with the TODO app is awesome!

- "People are successful in this team." It seems like there is so many chances, I think I need help choosing and maybe getting things to the end. 

- "Good omens" I had it on my radar for a long time. I love Neil Geilmann and Terry Pratchett

- The way that the course is structured in digipen sounds awesome! Half art and half programming. 
  
- Implementing your own ray tracing sounds super interesting
  - making a scene and graphics, etc.
  - 68 hours running
  - Working at the same time + studying sounds really tough
  - Impostor syndrome trigger
  - sounds like a tough time! working at the same is no easy task

- Earthbound 1st one is rough maaaaan.
  - Yeah, nintendo is very litigious
  - I know there is a fan based patch for translation patch for mother 3

## Work

- Yesterday Chris was asking about how to write a Java sample, which seemed like duplication
  - He wanted to showcase async stuff v/s sync 

- I am like 70% convinced that this stuff is duplicating the world xD
  - (tin foil hat) I think that Travis's silence is kind of conspicuous here.

- AI CLI being like SPX 
  - comma separated and bit flags (sounds very C-like)
  - Why not something more like trait driven

- I am a little bit worries that we are not able to come to an agreement on concepts
  - "accountability"

- GenAI stuff. Seems to be falling by the side
  - Travis seems to be super done at least with the initial release of assistants, but Jarno seems involved somehow on it

- I have flash backs of leadership saying something about being more top-down

- I think that if there isn't a technical reason against a single mono repo, we should do what we want.
  - I have no idea what sparse checkouts are, that's interesting. From a cursory read it looks really nice. We'd need to signal that in the main readme very clearly, not sure if it's a popular feature.

- Totally agree with you, scenario should be above language in terms of structure hierarchy. CCustomers know what they want, then they just look for the alternative they are familiar with.

## Bouldering

- They use colours everywhere, but they generally map to different values in more standardized ratings (7a+, 6b-: american, 0 - 13 in europe)

- Bouldering is super busy. It's annoying man, toally hear you. Queuing up, falling on someone. Nice that you have a lot of options!
