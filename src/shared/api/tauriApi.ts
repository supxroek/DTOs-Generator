import { invoke } from '@tauri-apps/api/core';
import type {
  CodegenOptions,
  ColumnSpec,
  ConnectionConfig,
  GeneratedCodeOutput,
  SaveCsFileRequest,
  SavedFileInfo,
  TableInfo,
} from '../types/dtoGen';

export const tauriApi = {
  testConnection(config: ConnectionConfig): Promise<boolean> {
    return invoke('cmd_test_connection', { config });
  },

  listTables(config: ConnectionConfig): Promise<TableInfo[]> {
    return invoke('cmd_list_tables', { config });
  },

  describeTable(
    config: ConnectionConfig,
    schema: string,
    tableName: string
  ): Promise<ColumnSpec[]> {
    return invoke('cmd_describe_table', { config, schema, tableName });
  },

  describeQuery(config: ConnectionConfig, sql: string): Promise<ColumnSpec[]> {
    return invoke('cmd_describe_query', { config, sql });
  },

  generateDto(
    columns: ColumnSpec[],
    options: CodegenOptions
  ): Promise<GeneratedCodeOutput> {
    return invoke('cmd_generate_dto', { columns, options });
  },

  saveCsFile(req: SaveCsFileRequest): Promise<SavedFileInfo> {
    return invoke('cmd_save_cs_file', { req });
  },

  listDatabases(config: ConnectionConfig): Promise<string[]> {
    return invoke('cmd_list_databases', { config });
  },
};