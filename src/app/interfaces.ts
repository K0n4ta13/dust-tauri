type TokenValue = { [key: string]: string | number | boolean };

export interface Token {
  type: string;
  value: string;
}

type ParsedToken = [string, string] | [TokenValue, string];

export interface TokenArray extends Array<ParsedToken> {}
