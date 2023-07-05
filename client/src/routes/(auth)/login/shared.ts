import type { UserErrorCode } from '$lib/graphql/schema';

export enum LoginError {
  MissingCredentials = 'MISSING_CREDENTIALS',
}
export type ErrorMessages = {
  [LoginError.MissingCredentials]: string;
  [UserErrorCode.Unauthorized]: string;
};
