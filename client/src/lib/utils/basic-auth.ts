/**
 * Credentials for a HTTP Basic Authentication instance
 */
export type Credentials = {
  username: string;
  password: string;
};

export const CREDENTIALS_REGEXP =
  /^ *(?:[Bb][Aa][Ss][Ii][Cc]) +([A-Za-z0-9._~+/-]+=*) *$/;
export const CRED_PAIR_REGEXP = /^([^:]*):(.*)$/;

/**
 * Creates a HTTP Basic compliant Header Value.
 *
 * This is only compatible on Browser Environment due to the fact that uses
 * the `Window`'s object to access `btoa`.
 *
 * @param username
 * @param password
 * @returns
 */
export function createHeader(username: string, password: string): string {
  const credentials = window.btoa(`${username}:${password}`);

  return `Basic ${credentials}`;
}

/**
 * Parses a `Request`'s object `Authorization` header to retrieve the
 * basic authentication credentials.
 *
 * @param request
 * @returns
 */
export function parseHeader(request: Request): Credentials {
  if (!request) {
    throw new TypeError('"request" argument is required');
  }

  if (typeof request !== 'object') {
    throw new TypeError('"request" must be an object');
  }

  const header = getAuthorizationHeader(request);

  if (!header) {
    throw new TypeError(
      '"request" headers doesn\'t include an Authorization key pair',
    );
  }

  return parse(header);
}

export function decodeBase64(str: string) {
  return Buffer.from(str, 'base64').toString();
}

export function getAuthorizationHeader(request: Request): string | null {
  if (!request.headers || typeof request.headers !== 'object') {
    throw new TypeError(
      'argument "request" must be an object with the "headers" field',
    );
  }

  if (typeof request.headers.get !== 'function') {
    throw new TypeError(
      'Missing "get" function for "Headers" on "Request.headers"',
    );
  }

  return request.headers.get('authorization') || null;
}

function parse(authorization: string): Credentials {
  if (typeof authorization !== 'string') {
    throw new TypeError('"authorization" in not of type "string"');
  }

  const matches = CREDENTIALS_REGEXP.exec(authorization);

  if (!matches) {
    throw new TypeError(
      "The provided Authorization Header doesn't follows HTTP Basic Authentication",
    );
  }

  const creds = CRED_PAIR_REGEXP.exec(decodeBase64(matches[1]));

  if (!creds) {
    throw new TypeError('Failed to retrieve user credentials');
  }

  return {
    username: creds[1],
    password: creds[2],
  };
}
