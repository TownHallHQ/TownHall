export enum StatusCode {
  Ok = 200,
  BadRequest = 400,
  Unauthorized = 401,
  Forbidden = 403,
  NotFound = 404,
  InternalServerError = 500,
}

export type HttpErrorResponse<C> = {
  statusCode: StatusCode;
  message: string | C;
  code: C;
}

export class JsonResponse {
  static error<C>(statusCode: StatusCode, code: C, message?: string): Response {
    const object: HttpErrorResponse<C> = {
      statusCode,
      message: message ?? code,
      code,
    };

    return new Response(JSON.stringify(object), {
      status: statusCode,
    });
  }

  static success<T>(data: T, statusCode: StatusCode = StatusCode.Ok): Response {
    const object = {
      statusCode,
      data,
    };

    return new Response(JSON.stringify(object), {
      status: statusCode,
    });
  }
}
