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
  Email: { input: any; output: any; }
  Pxid: { input: any; output: any; }
  Upload: { input: any; output: any; }
};

export type AccessToken = {
  __typename?: 'AccessToken';
  accessToken: Scalars['String']['output'];
};

export type AuthError = {
  __typename?: 'AuthError';
  code: AuthErrorCode;
  message: Scalars['String']['output'];
};

export enum AuthErrorCode {
  EmailParseError = 'EMAIL_PARSE_ERROR',
  EmailTaken = 'EMAIL_TAKEN',
  Internal = 'INTERNAL',
  Unauthorized = 'UNAUTHORIZED'
}

export type Image = {
  __typename?: 'Image';
  id: Scalars['Pxid']['output'];
  url: Scalars['String']['output'];
};

export type Me = {
  __typename?: 'Me';
  error?: Maybe<AuthError>;
  user?: Maybe<User>;
};

export type MutationRoot = {
  __typename?: 'MutationRoot';
  /** Creates a post authored by the user identified by the provided token */
  postCreate: PostCreate;
  tokenCreate: TokenCreate;
  userAvatarUpdate: UserAvatarUpdate;
  userFollow: UserFollow;
  userRegister: UserRegister;
  userUnfollow: UserUnfollow;
  userUpdate: UserUpdate;
};


export type MutationRootPostCreateArgs = {
  input: PostCreateInput;
};


export type MutationRootTokenCreateArgs = {
  email: Scalars['Email']['input'];
  password: Scalars['String']['input'];
};


export type MutationRootUserAvatarUpdateArgs = {
  file: Scalars['Upload']['input'];
};


export type MutationRootUserFollowArgs = {
  followeeId: Scalars['Pxid']['input'];
};


export type MutationRootUserRegisterArgs = {
  input: UserRegisterInput;
};


export type MutationRootUserUnfollowArgs = {
  followeeId: Scalars['Pxid']['input'];
};


export type MutationRootUserUpdateArgs = {
  input: UserUpdateInput;
};

/** Information about pagination in a connection */
export type PageInfo = {
  __typename?: 'PageInfo';
  /** When paginating forwards, the cursor to continue. */
  endCursor?: Maybe<Scalars['String']['output']>;
  /** When paginating forwards, are there more items? */
  hasNextPage: Scalars['Boolean']['output'];
  /** When paginating backwards, are there more items? */
  hasPreviousPage: Scalars['Boolean']['output'];
  /** When paginating backwards, the cursor to continue. */
  startCursor?: Maybe<Scalars['String']['output']>;
};

export type Post = {
  __typename?: 'Post';
  author: User;
  authorId: Scalars['Pxid']['output'];
  content?: Maybe<Scalars['String']['output']>;
  createdAt: Scalars['DateTime']['output'];
  head: Scalars['Boolean']['output'];
  id: Scalars['Pxid']['output'];
  parentId?: Maybe<Scalars['Pxid']['output']>;
  title: Scalars['String']['output'];
  updatedAt: Scalars['DateTime']['output'];
};

export type PostConnection = {
  __typename?: 'PostConnection';
  /** A list of edges. */
  edges: Array<PostEdge>;
  /** A list of nodes. */
  nodes: Array<Post>;
  /** Information to aid in pagination. */
  pageInfo: PageInfo;
  totalCount: Scalars['Int']['output'];
};

export type PostCreate = {
  __typename?: 'PostCreate';
  error?: Maybe<PostError>;
  post?: Maybe<Post>;
};

export type PostCreateInput = {
  content?: InputMaybe<Scalars['String']['input']>;
  parentId?: InputMaybe<Scalars['Pxid']['input']>;
  title: Scalars['String']['input'];
};

/** An edge in a connection. */
export type PostEdge = {
  __typename?: 'PostEdge';
  /** A cursor for use in pagination */
  cursor: Scalars['String']['output'];
  /** The item at the end of the edge */
  node: Post;
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
  posts: PostConnection;
  user: UserConnection;
};


export type QueryRootPostsArgs = {
  after?: InputMaybe<Scalars['Pxid']['input']>;
  before?: InputMaybe<Scalars['Pxid']['input']>;
  first?: InputMaybe<Scalars['Int']['input']>;
  last?: InputMaybe<Scalars['Int']['input']>;
};


export type QueryRootUserArgs = {
  after?: InputMaybe<Scalars['Pxid']['input']>;
  before?: InputMaybe<Scalars['Pxid']['input']>;
  filter?: InputMaybe<UserFilterInput>;
  first?: InputMaybe<Scalars['Int']['input']>;
  last?: InputMaybe<Scalars['Int']['input']>;
};

export type TokenCreate = {
  __typename?: 'TokenCreate';
  error?: Maybe<AuthError>;
  token?: Maybe<AccessToken>;
};

/** A Platform's User */
export type User = {
  __typename?: 'User';
  avatar?: Maybe<Image>;
  createdAt: Scalars['DateTime']['output'];
  email: Scalars['String']['output'];
  id: Scalars['Pxid']['output'];
  name: Scalars['String']['output'];
  /** Lists posts authored by this user */
  posts: PostConnection;
  surname: Scalars['String']['output'];
  updatedAt: Scalars['DateTime']['output'];
  username: Scalars['String']['output'];
};


/** A Platform's User */
export type UserPostsArgs = {
  after?: InputMaybe<Scalars['Pxid']['input']>;
  before?: InputMaybe<Scalars['Pxid']['input']>;
  first?: InputMaybe<Scalars['Int']['input']>;
  last?: InputMaybe<Scalars['Int']['input']>;
};

export type UserAvatarUpdate = {
  __typename?: 'UserAvatarUpdate';
  error?: Maybe<UserError>;
  user?: Maybe<User>;
};

export type UserConnection = {
  __typename?: 'UserConnection';
  /** A list of edges. */
  edges: Array<UserEdge>;
  /** A list of nodes. */
  nodes: Array<User>;
  /** Information to aid in pagination. */
  pageInfo: PageInfo;
  totalCount: Scalars['Int']['output'];
};

/** An edge in a connection. */
export type UserEdge = {
  __typename?: 'UserEdge';
  /** A cursor for use in pagination */
  cursor: Scalars['String']['output'];
  /** The item at the end of the edge */
  node: User;
};

export type UserError = {
  __typename?: 'UserError';
  code: UserErrorCode;
  message: Scalars['String']['output'];
};

export enum UserErrorCode {
  AvatarUploadError = 'AVATAR_UPLOAD_ERROR',
  EmailTaken = 'EMAIL_TAKEN',
  Internal = 'INTERNAL',
  InvalidEmail = 'INVALID_EMAIL',
  Unauthorized = 'UNAUTHORIZED'
}

export type UserFilterInput = {
  email?: InputMaybe<Scalars['String']['input']>;
  id?: InputMaybe<Scalars['Pxid']['input']>;
  username?: InputMaybe<Scalars['String']['input']>;
};

export type UserFollow = {
  __typename?: 'UserFollow';
  error?: Maybe<UserError>;
};

export type UserRegister = {
  __typename?: 'UserRegister';
  error?: Maybe<UserError>;
  user?: Maybe<User>;
};

export type UserRegisterInput = {
  email: Scalars['Email']['input'];
  name: Scalars['String']['input'];
  password: Scalars['String']['input'];
  surname: Scalars['String']['input'];
  username: Scalars['String']['input'];
};

export type UserUnfollow = {
  __typename?: 'UserUnfollow';
  error?: Maybe<UserError>;
};

export type UserUpdate = {
  __typename?: 'UserUpdate';
  error?: Maybe<UserError>;
  user?: Maybe<User>;
};

export type UserUpdateInput = {
  name?: InputMaybe<Scalars['String']['input']>;
  surname?: InputMaybe<Scalars['String']['input']>;
};

export type TokenCreateMutationVariables = Exact<{
  email: Scalars['Email']['input'];
  password: Scalars['String']['input'];
}>;


export type TokenCreateMutation = { __typename?: 'MutationRoot', tokenCreate: { __typename?: 'TokenCreate', token?: { __typename?: 'AccessToken', accessToken: string } | null, error?: { __typename?: 'AuthError', code: AuthErrorCode, message: string } | null } };

export type AuthorFragment = { __typename?: 'User', id: any, name: string, surname: string, username: string, email: string, createdAt: any, updatedAt: any };

export type PostCreateMutationVariables = Exact<{
  input: PostCreateInput;
}>;


export type PostCreateMutation = { __typename?: 'MutationRoot', postCreate: { __typename?: 'PostCreate', post?: { __typename?: 'Post', id: any, authorId: any, parentId?: any | null, head: boolean, title: string, content?: string | null, createdAt: any, updatedAt: any } | null, error?: { __typename?: 'PostError', code: PostErrorCode, message: string } | null } };

export type PostCreateFieldsFragment = { __typename?: 'Post', id: any, authorId: any, parentId?: any | null, head: boolean, title: string, content?: string | null, createdAt: any, updatedAt: any };

export type PostListQueryVariables = Exact<{
  first?: InputMaybe<Scalars['Int']['input']>;
  last?: InputMaybe<Scalars['Int']['input']>;
  after?: InputMaybe<Scalars['Pxid']['input']>;
  before?: InputMaybe<Scalars['Pxid']['input']>;
}>;


export type PostListQuery = { __typename?: 'QueryRoot', posts: { __typename?: 'PostConnection', edges: Array<{ __typename?: 'PostEdge', node: { __typename?: 'Post', id: any, authorId: any, parentId?: any | null, head: boolean, title: string, content?: string | null, createdAt: any, updatedAt: any, author: { __typename?: 'User', id: any, name: string, surname: string, username: string, email: string, createdAt: any, updatedAt: any } } }>, pageInfo: { __typename?: 'PageInfo', hasPreviousPage: boolean, hasNextPage: boolean, startCursor?: string | null, endCursor?: string | null } } };

export type PostListFieldsFragment = { __typename?: 'Post', id: any, authorId: any, parentId?: any | null, head: boolean, title: string, content?: string | null, createdAt: any, updatedAt: any, author: { __typename?: 'User', id: any, name: string, surname: string, username: string, email: string, createdAt: any, updatedAt: any } };

export type CurrentUserFragment = { __typename?: 'User', id: any, name: string, surname: string, email: string, username: string, createdAt: any, updatedAt: any, avatar?: { __typename?: 'Image', id: any, url: string } | null };

export type MeQueryVariables = Exact<{ [key: string]: never; }>;


export type MeQuery = { __typename?: 'QueryRoot', me: { __typename?: 'Me', user?: { __typename?: 'User', id: any, name: string, surname: string, email: string, username: string, createdAt: any, updatedAt: any, avatar?: { __typename?: 'Image', id: any, url: string } | null } | null } };

export type UserAvatarUpdateMutationVariables = Exact<{
  file: Scalars['Upload']['input'];
}>;


export type UserAvatarUpdateMutation = { __typename?: 'MutationRoot', userAvatarUpdate: { __typename?: 'UserAvatarUpdate', user?: { __typename?: 'User', id: any, name: string, surname: string, email: string, username: string, createdAt: any, updatedAt: any, avatar?: { __typename?: 'Image', id: any, url: string } | null } | null, error?: { __typename?: 'UserError', code: UserErrorCode, message: string } | null } };

export type UserPostsListQueryVariables = Exact<{
  username?: InputMaybe<Scalars['String']['input']>;
  first?: InputMaybe<Scalars['Int']['input']>;
  last?: InputMaybe<Scalars['Int']['input']>;
  after?: InputMaybe<Scalars['Pxid']['input']>;
  before?: InputMaybe<Scalars['Pxid']['input']>;
}>;


export type UserPostsListQuery = { __typename?: 'QueryRoot', user: { __typename?: 'UserConnection', edges: Array<{ __typename?: 'UserEdge', node: { __typename?: 'User', id: any, name: string, surname: string, username: string, email: string, createdAt: any, updatedAt: any, posts: { __typename?: 'PostConnection', edges: Array<{ __typename?: 'PostEdge', node: { __typename?: 'Post', id: any, authorId: any, parentId?: any | null, head: boolean, title: string, content?: string | null, createdAt: any, updatedAt: any, author: { __typename?: 'User', id: any, name: string, surname: string, username: string, email: string, createdAt: any, updatedAt: any } } }>, pageInfo: { __typename?: 'PageInfo', hasPreviousPage: boolean, hasNextPage: boolean, startCursor?: string | null, endCursor?: string | null } } } }>, pageInfo: { __typename?: 'PageInfo', hasPreviousPage: boolean, hasNextPage: boolean, startCursor?: string | null, endCursor?: string | null } } };

export type UserRegisterMutationVariables = Exact<{
  input: UserRegisterInput;
}>;


export type UserRegisterMutation = { __typename?: 'MutationRoot', userRegister: { __typename?: 'UserRegister', user?: { __typename?: 'User', id: any, name: string, surname: string, email: string, username: string, createdAt: any, updatedAt: any, avatar?: { __typename?: 'Image', id: any, url: string } | null } | null, error?: { __typename?: 'UserError', code: UserErrorCode, message: string } | null } };

export const PostCreateFieldsFragmentDoc = gql`
    fragment PostCreateFields on Post {
  id
  authorId
  parentId
  head
  title
  content
  createdAt
  updatedAt
}
    `;
export const AuthorFragmentDoc = gql`
    fragment Author on User {
  id
  name
  surname
  username
  email
  createdAt
  updatedAt
}
    `;
export const PostListFieldsFragmentDoc = gql`
    fragment PostListFields on Post {
  id
  authorId
  parentId
  head
  title
  content
  createdAt
  updatedAt
  author {
    ...Author
  }
}
    ${AuthorFragmentDoc}`;
export const CurrentUserFragmentDoc = gql`
    fragment CurrentUser on User {
  id
  name
  surname
  email
  username
  avatar {
    id
    url
  }
  createdAt
  updatedAt
}
    `;
export const TokenCreateDocument = gql`
    mutation TokenCreate($email: Email!, $password: String!) {
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
export const PostCreateDocument = gql`
    mutation PostCreate($input: PostCreateInput!) {
  postCreate(input: $input) {
    post {
      ...PostCreateFields
    }
    error {
      code
      message
    }
  }
}
    ${PostCreateFieldsFragmentDoc}`;
export const PostListDocument = gql`
    query PostList($first: Int, $last: Int, $after: Pxid, $before: Pxid) {
  posts(first: $first, last: $last, after: $after, before: $before) {
    edges {
      node {
        ...PostListFields
      }
    }
    pageInfo {
      hasPreviousPage
      hasNextPage
      startCursor
      endCursor
    }
  }
}
    ${PostListFieldsFragmentDoc}`;
export const MeDocument = gql`
    query Me {
  me {
    user {
      ...CurrentUser
    }
  }
}
    ${CurrentUserFragmentDoc}`;
export const UserAvatarUpdateDocument = gql`
    mutation UserAvatarUpdate($file: Upload!) {
  userAvatarUpdate(file: $file) {
    user {
      ...CurrentUser
    }
    error {
      code
      message
    }
  }
}
    ${CurrentUserFragmentDoc}`;
export const UserPostsListDocument = gql`
    query UserPostsList($username: String, $first: Int, $last: Int, $after: Pxid, $before: Pxid) {
  user(filter: {username: $username}) {
    edges {
      node {
        ...Author
        posts(first: $first, last: $last, after: $after, before: $before) {
          edges {
            node {
              ...PostListFields
            }
          }
          pageInfo {
            hasPreviousPage
            hasNextPage
            startCursor
            endCursor
          }
        }
      }
    }
    pageInfo {
      hasPreviousPage
      hasNextPage
      startCursor
      endCursor
    }
  }
}
    ${AuthorFragmentDoc}
${PostListFieldsFragmentDoc}`;
export const UserRegisterDocument = gql`
    mutation UserRegister($input: UserRegisterInput!) {
  userRegister(input: $input) {
    user {
      ...CurrentUser
    }
    error {
      code
      message
    }
  }
}
    ${CurrentUserFragmentDoc}`;