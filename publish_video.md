# Publishing


The Video API allows you to publish [Videos](https://developers.facebook.com/docs/graph-api/reference/video) and [Reels](https://developers.facebook.com/documentation/video-api/guides/reels-publishing) on [Facebook Pages](https://developers.facebook.com/docs/graph-api/reference/page).

## Requirements

To publish a video on a Page you will need:

* A Page access token requested by a person who can perform the `CREATE_CONTENT` task on the Page
* The person requesting the token must grant your app access to the following permissions via Facebook Login:
    * [`pages_show_list`](https://developers.facebook.com/docs/permissions/reference/pages_show_list)
    * [`pages_read_engagement`](https://developers.facebook.com/docs/permissions/reference/pages_read_engagement)
    * [`pages_manage_posts`](https://developers.facebook.com/docs/permissions/reference/pages_manage_posts)
* A video handle
    * This handle is received when you upload your video file to Meta servers using the [Resumable Upload API](#upload-a-video-file)

## Publish a video

To publish a video send a `POST` request to the `/<PAGE_ID>/videos` endpoint with

```curl
curl -X POST \
  "https://graph-video.facebook.com/v25.0/<PAGE_ID>/videos" \
  -F "access_token=<PAGE_ACCESS_TOKEN>" \
  -F "title=<VIDEO_TITLE>" \
  -F "description=<VIDEO_DESCRIPTION>" \
  -F "fbuploader_video_file_chunk=<UPLOADED_FILE_HANDLE>"
```

On success, your app receives a JSON response with the Video ID.

```json
{
  "id":"<VIDEO_ID>"
}
```

## Upload a video file

**Note:** The following content is from the [Resumable Upload API documentation](https://developers.facebook.com/docs/graph-api/guides/upload).

The Resumable Upload API allows you to upload large files to Meta's social graph and resume interrupted upload sessions without having to start over. Once you have uploaded your file, you can publish it.

References for endpoints that support uploaded file handles will indicate if the endpoints support handles returned by the Resumable Upload API.

### Before you start

This guide assumes you have read  the [Graph API Overview](https://developers.facebook.com/docs/graph-api/overview) and the [Meta Development](https://developers.facebook.com/documentation/development) guides and performed the necessary actions needed to develop with Meta.

You will need:

* A Meta app ID
* A file in one of the following formats:
    * `pdf`
    * `jpeg`
    * `jpg`
    * `png`
    * `mp4`
* A User access token

## Step 1: Start an upload session {#step-1}

To start an upload session send a `POST` request to the `/<APP_ID>/uploads` endpoint, where `<APP_ID>` is your app's Meta ID, with the following required parameters:

* `file_name` - the name of your file
* `file_length` - file size in bytes
* `file_type` - The file's MIME type. Valid values are: `application/pdf`, `image/jpeg`, `image/jpg`, `image/png`, and `video/mp4`

#### Request Syntax

_Formatted for readability._

```http
curl -i -X POST "https://graph.facebook.com/v25.0/<APP_ID>/uploads
  ?file_name=<FILE_NAME>
  &file_length=<FILE_LENGTH>
  &file_type=<FILE_TYPE>
  &access_token=<USER_ACCESS_TOKEN>"
```

Upon success, your app will receive a JSON response with the upload session ID.

```json
{
  "id": "upload:<UPLOAD_SESSION_ID>"
}
```

## Step 2: Start the upload {#step-2}

Start uploading the file by sending a `POST` request to the `/upload:<UPLOAD_SESSION_ID>` endpoint with the following `file_offset` set to `0`.

#### Request Syntax

```html
curl -i -X POST "https://graph.facebook.com/v25.0/upload:<UPLOAD_SESSION_ID>"
  --header "Authorization: OAuth <USER_ACCESS_TOKEN>"
  --header "file_offset: 0"
  --data-binary @<FILE_NAME>
```

You must include the access token in the header or the call will fail.

On success, your app receives the file handle which you will use in your API calls to publish the file to your endpoint.

```json
{
  "h": "<UPLOADED_FILE_HANDLE>"
}
```

#### Sample Response

```json
{
    "h": "2:c2FtcGxl..."
}
```

### Resume an interrupted upload

If you have initiated an upload session but it is taking longer than expected or has been interrupted, send a `GET` request to the `/upload:<UPLOAD_SESSION_ID>` endpoint from [Step 1](#step-1).

```html
curl -i -X GET "https://graph.facebook.com/v25.0/upload:<UPLOAD_SESSION_ID>"
  --header "Authorization: OAuth <USER_ACCESS_TOKEN>"
```

Upon success, your app will receive a JSON response with the `file_offset` value that you can use to resume the upload process from the point of interruption.

```json
{
  "id": "upload:<UPLOAD_SESSION_ID>"
  "file_offset": "<FILE_OFFSET>"
}
```

Send another `POST` request, like the you sent in [Step 2](#step-2), with `file_offset` set to this `file_offset` value you just received.  This will resume the upload process from the point of interruption.

```html
curl -i -X POST "https://graph.facebook.com/v25.0/upload:<UPLOAD_SESSION_ID>"
  --header "Authorization: OAuth <USER_ACCESS_TOKEN>"
  --header "file_offset: <FILE_OFFSET>"
  --data-binary @<FILE_NAME>
```

## Next Steps

* Visit the [Video API documentation](https://developers.facebook.com/documentation/video-api/guides/publishing) to publish a video to a Facebook Page.


