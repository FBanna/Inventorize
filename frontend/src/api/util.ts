

export class StatusError extends Error {
  constructor(
    message: any,
    public status?: number
  ) {
    super(message);
    this.name = "StatusError";
  }
}


export async function fetchURL(
  url: string,
  opts: ApiOpts
): Promise<Response> {

  opts = opts || {};
  opts.headers = opts.headers || {};

  const { headers, ...rest } = opts;
  let res;
  try {
    res = await fetch(`${import.meta.env.VITE_API_URL}${url}`, {
      redirect: "follow",
      headers: {
        ...headers,
      },
      ...rest,
    });
  } catch (e) {


    throw new StatusError("000 No connection", 0);

  }

  if (200 <= res.status && res.status < 400) {

    return res;
    
  }

  const body = await res.text();

  const error = new StatusError(
      body || `${res.status} ${res.statusText}`,
      res.status
  );


  throw error;
}

export async function fetchJSON<T>(
  url: string,
  opts: ApiOpts
): Promise<T> {

  const res = await fetchURL(url, opts)
  return res.json() as Promise<T>

}