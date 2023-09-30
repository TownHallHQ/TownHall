export class GraphQLError<C> extends Error {
  private readonly _code: C;
  private readonly _message: string;

  protected constructor(name: string, code: C, message: string) {
    super(`${code}: ${message}`);

    this.name = name;
    this._code = code;
    this._message = message;
  }

  get code(): C {
    return this._code;
  }

  get message(): string {
    return this._message;
  }

  public static new<C>(code: C, message?: string): GraphQLError<C> {
    return new GraphQLError<C>(
      this.name,
      code,
      message || `${this.name}: ${code}`,
    );
  }

  public toObject(): { error: string; code: C; message: string } {
    return {
      error: this.name,
      code: this._code,
      message: this._message,
    };
  }

  public toString(): string {
    return JSON.stringify(this.toObject());
  }
}
