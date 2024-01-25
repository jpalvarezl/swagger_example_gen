# OpenAI Assistants

## Needed dependencies

- `curl`
- `jq` (optional)

## Tips

- For streaming scenarios `jq` doesn't work, since it's not technically JSON that comes back (as a single response). In that case is better to dump into a file by adding ` > dump.json` after the `curl` call
- Be mindful of trailing space after `\` (that took me too much time to figure out)

## Curl template

Notice the authentication is different depending on whether we are using Azure or not.

### Upload file

Sending a text file, will make the `Content-Type` for the `file` segment `text/plain`

```bash
curl --trace-ascii ./traces/assistant_text_upload.trace  ${OPENAI_ENDPOINT}/files \
-H "Authorization: Bearer ${OPENAI_KEY}" \
-F purpose="assistants"  -F file="@sample_files/text.txt"
```

Sending an image file will make the `Content-Type` for the `file` segment `image/png`

```bash
curl --trace-ascii ./traces/assistant_image_upload.trace  ${OPENAI_ENDPOINT}/files \
-H "Authorization: Bearer ${OPENAI_KEY}" \
-F purpose="assistants"  -F file="@sample_files/ms_logo.png"
```

Sending an audio file will make the `Content-Type` will result in an error as it's not supported yet.

```bash
curl --trace-ascii ./traces/assistant_audio_upload.trace  ${OPENAI_ENDPOINT}/files \
-H "Authorization: Bearer ${OPENAI_KEY}" \
-F purpose="assistants"  -F file="@sample_files/batman.wav"
```
