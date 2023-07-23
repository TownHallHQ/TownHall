import gql from 'graphql-tag';
export type Maybe<T> = T | null;
export type InputMaybe<T> = Maybe<T>;
export type Exact<T extends { [key: string]: unknown }> = { [K in keyof T]: T[K] };
export type MakeOptional<T, K extends keyof T> = Omit<T, K> & { [SubKey in K]?: Maybe<T[SubKey]> };
export type MakeMaybe<T, K extends keyof T> = Omit<T, K> & { [SubKey in K]: Maybe<T[SubKey]> };
export type MakeEmpty<T extends { [key: string]: unknown }, K extends keyof T> = { [_ in K]?: never };
export type Incremental<T> = T | { [P in keyof T]?: P extends ' $fragmentName' | '__typename' ? T[P] : never };
export type Omit<T, K extends keyof T> = Pick<T, Exclude<keyof T, K>>;
/** All built-in and custom scalars, mapped to their actual values */
export type Scalars = {
  ID: { input: string; output: string; }
  String: { input: string; output: string; }
  Boolean: { input: boolean; output: boolean; }
  Int: { input: number; output: number; }
  Float: { input: number; output: number; }
  DateTime: { input: any; output: any; }
  Pxid: { input: any; output: any; }
};

export type AccessToken = {
  __typename?: 'AccessToken';
  accessToken: Scalars['String']['output'];
};

export type Me = {
  __typename?: 'Me';
  error?: Maybe<UserError>;
  user?: Maybe<User>;
};

export type MutationRoot = {
  __typename?: 'MutationRoot';
  /** Creates a post authored by the user identified by the provided token */
  postCreate: PostCreate;
  tokenCreate: TokenCreate;
  userRegister: UserRegister;
};


export type MutationRootPostCreateArgs = {
  input: PostCreateInput;
};


export type MutationRootTokenCreateArgs = {
  email: Scalars['String']['input'];
  password: Scalars['String']['input'];
};


export type MutationRootUserRegisterArgs = {
  input: UserRegisterInput;
};

export type Post = {
  __typename?: 'Post';
  authorId: Scalars['Pxid']['output'];
  content: Scalars['String']['output'];
  createdAt: Scalars['DateTime']['output'];
  head: Scalars['Boolean']['output'];
  id: Scalars['Pxid']['output'];
  parentId?: Maybe<Scalars['Pxid']['output']>;
  title: Scalars['String']['output'];
  updatedAt: Scalars['DateTime']['output'];
};

export type PostCreate = {
  __typename?: 'PostCreate';
  error?: Maybe<PostError>;
  post?: Maybe<Post>;
};

export type PostCreateInput = {
  content: Scalars['String']['input'];
  parentId?: InputMaybe<Scalars['Pxid']['input']>;
  title: Scalars['String']['input'];
};

export type PostError = {
  __typename?: 'PostError';
  code: PostErrorCode;
  message: Scalars['String']['output'];
};

export enum PostErrorCode {
  InvalidParentId = 'INVALID_PARENT_ID',
  Unauthorized = 'UNAUTHORIZED',
  Unknown = 'UNKNOWN'
}

export type QueryRoot = {
  __typename?: 'QueryRoot';
  me: Me;
};

export type TokenCreate = {
  __typename?: 'TokenCreate';
  error?: Maybe<UserError>;
  token?: Maybe<AccessToken>;
};

/** A Platform's User */
export type User = {
  __typename?: 'User';
  createdAt: Scalars['DateTime']['output'];
  email: Scalars['String']['output'];
  id: Scalars['Pxid']['output'];
  name: Scalars['String']['output'];
  surname: Scalars['String']['output'];
  updatedAt: Scalars['DateTime']['output'];
  username: Scalars['String']['output'];
};

export type UserError = {
  __typename?: 'UserError';
  code: UserErrorCode;
  message: Scalars['String']['output'];
};

export enum UserErrorCode {
  EmailTaken = 'EMAIL_TAKEN',
  Internal = 'INTERNAL',
  InvalidEmail = 'INVALID_EMAIL',
  Unauthorized = 'UNAUTHORIZED'
}

export type UserRegister = {
  __typename?: 'UserRegister';
  error?: Maybe<UserError>;
  user?: Maybe<User>;
};

export type UserRegisterInput = {
  email: Scalars['String']['input'];
  name: Scalars['String']['input'];
  password: Scalars['String']['input'];
  surname: Scalars['String']['input'];
  username: Scalars['String']['input'];
};

export type GetCurrentUserQueryVariables = Exact<{ [key: string]: never; }>;


export type GetCurrentUserQuery = { __typename?: 'QueryRoot', me: { __typename?: 'Me', user?: { __typename?: 'User', id: any, name: string, surname: string, email: string, username: string, createdAt: any, updatedAt: any } | null } };

export type CurrentUserFragment = { __typename?: 'User', id: any, name: string, surname: string, email: string, username: string, createdAt: any, updatedAt: any };

export type TokenCreateMutationVariables = Exact<{
  email: Scalars['String']['input'];
  password: Scalars['String']['input'];
}>;


export type TokenCreateMutation = { __typename?: 'MutationRoot', tokenCreate: { __typename?: 'TokenCreate', token?: { __typename?: 'AccessToken', accessToken: string } | null, error?: { __typename?: 'UserError', code: UserErrorCode, message: string } | null } };

export type UserCreateMutationVariables = Exact<{
  input: UserRegisterInput;
}>;


export type UserCreateMutation = { __typename?: 'MutationRoot', userRegister: { __typename?: 'UserRegister', user?: { __typename?: 'User', id: any } | null, error?: { __typename?: 'UserError', code: UserErrorCode, message: string } | null } };

export const CurrentUserFragmentDoc = gql`
    fragment CurrentUser on User {
  id
  name
  surname
  email
  username
  createdAt
  updatedAt
}
    `;
export const GetCurrentUserDocument = gql`
    query GetCurrentUser {
  me {
    user {
      ...CurrentUser
    }
  }
}
    ${CurrentUserFragmentDoc}`;
export const TokenCreateDocument = gql`
    mutation TokenCreate($email: String!, $password: String!) {
  tokenCreate(email: $email, password: $password) {
    token {
      accessToken
    }
    error {
      code
      message
    }
  }
}
    `;
export const UserCreateDocument = gql`
    mutation UserCreate($input: UserRegisterInput!) {
  userRegister(input: $input) {
    user {
      id
    }
    error {
      code
      message
    }
  }
}
    `;