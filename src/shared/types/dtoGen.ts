export interface ConnectionConfig {
  host: string;
  port: number;
  database: string;
  username: string;
  password: string;
  trustCert: boolean;
}

export interface TableInfo {
  schema: string;
  name: string;
}

export type SqlTypeKind =
  | 'varChar'
  | 'nVarChar'
  | 'int'
  | 'bigInt'
  | 'smallInt'
  | 'tinyInt'
  | 'decimal'
  | 'float'
  | 'real'
  | 'bit'
  | 'dateTime'
  | 'date'
  | 'uniqueIdentifier'
  | 'varBinary'
  | { other: string };

export interface ColumnSpec {
  rawName: string;
  sqlType: SqlTypeKind;
  isNullable: boolean;
}

export type CSharpTarget = 'netFramework4x' | 'netModern';

export interface CodegenOptions {
  className: string;
  namespace: string;
  target: CSharpTarget;
  tableVarName: string;
  rowVarName: string;
}

export interface GeneratedCodeOutput {
  dtoCode: string;
  mapperCode: string;
}

export interface SaveCsFileRequest {
  folderPath: string;
  fileName: string;
  content: string;
  overwrite: boolean;
}

export interface SavedFileInfo {
  fileName: string;
  fullPath: string;
  bytesWritten: number;
}