type ApiMethod = "GET" | "POST" | "PUT" | "DELETE" | "PATCH";

// type ApiContent =
//   | Blob
//   | File
//   | Pick<ReadableStreamDefaultReader<any>, "read">
//   | "";

interface ApiOpts {
  method?: ApiMethod;
  headers?: object;
  body?: any;
  signal?: AbortSignal;
}