<script setup lang="ts">
import { ref } from 'vue';
import ConnectionBar from './features/connection/ConnectionBar.vue';
import SchemaExplorer from './features/schema-explorer/SchemaExplorer.vue';
import CodegenPanel from './features/codegen/CodegenPanel.vue';
import { tauriApi } from './shared/api/tauriApi';
import type { ColumnSpec, ConnectionConfig, TableInfo } from './shared/types/dtoGen';

const config = ref<ConnectionConfig>({
  host: 'spc805srv',
  port: 1433,
  database: '',
  username: 'sa',
  password: '',
  trustCert: true,
});

const databases = ref<string[]>([]);
const selectedDatabase = ref('');
const loadingTables = ref(false);

const tables = ref<TableInfo[]>([]);
const refreshKey = ref(0);
const selectedColumns = ref<ColumnSpec[]>([]);
const suggestedClassName = ref('UserDto');
const toast = ref<{ type: 'success' | 'error'; text: string } | null>(null);

function showToast(type: 'success' | 'error', text: string) {
  toast.value = { type, text };
}

async function loadTablesForCurrentDb() {
  if (!config.value.database.trim()) return;
  loadingTables.value = true;
  try {
    tables.value = await tauriApi.listTables(config.value);
    refreshKey.value += 1;
    showToast(
      'success',
      `โหลดรายการ Table/View จาก [${config.value.database}] สำเร็จ (${tables.value.length} รายการ)`
    );
  } catch (err: any) {
    showToast('error', err?.message ?? String(err));
  } finally {
    loadingTables.value = false;
  }
}

function handleDatabasesLoaded(dbs: string[]) {
  databases.value = dbs;
  selectedDatabase.value = '';
  tables.value = [];
  selectedColumns.value = [];
  showToast('success', `พบ ${dbs.length} ฐานข้อมูล กรุณาเลือกฐานข้อมูลที่ต้องการ`);
}

async function handleSelectDatabase() {
  if (!selectedDatabase.value) return;
  config.value.database = selectedDatabase.value;
  selectedColumns.value = [];
  await loadTablesForCurrentDb();
}
</script>

<template>
  <div class="min-h-screen bg-slate-100 p-4 text-slate-900">
    <div class="mx-auto max-w-7xl space-y-4">
      <header class="flex flex-wrap items-center justify-between gap-2">
        <div>
          <h1 class="text-lg font-bold text-slate-900">C# DTO & DataRow Mapper Generator</h1>
          <p class="text-xs text-slate-500">WinForms DTO Tool • Tauri + Vue + Tailwind + Bun + Rust</p>
        </div>

        <div
          v-if="toast"
          :class="
            toast.type === 'success'
              ? 'rounded-lg bg-emerald-50 px-3 py-1.5 text-xs font-medium text-emerald-700 ring-1 ring-emerald-200'
              : 'rounded-lg bg-rose-50 px-3 py-1.5 text-xs font-medium text-rose-700 ring-1 ring-rose-200'
          "
        >
          {{ toast.text }}
        </div>
      </header>

      <ConnectionBar
        v-model:config="config"
        @connected="loadTablesForCurrentDb"
        @databases-loaded="handleDatabasesLoaded"
        @error="(m) => showToast('error', m)"
      />

      <!-- Dropdown เลือก Database เมื่อเชื่อมต่อแบบไม่ระบุ Database -->
      <section
        v-if="databases.length > 0"
        class="flex flex-wrap items-center justify-between gap-3 rounded-xl border border-blue-200 bg-blue-50/70 px-4 py-3"
      >
        <div class="flex flex-wrap items-center gap-3">
          <label class="text-xs font-semibold text-blue-900">เลือก Database:</label>
          <select
            v-model="selectedDatabase"
            :disabled="loadingTables"
            @change="handleSelectDatabase"
            class="min-w-60 rounded-lg border border-blue-300 bg-white px-3 py-1.5 text-sm text-slate-800 focus:border-blue-500 focus:outline-none"
          >
            <option value="" disabled>-- เลือกฐานข้อมูล ({{ databases.length }}) --</option>
            <option v-for="db in databases" :key="db" :value="db">
              {{ db }}
            </option>
          </select>
        </div>

        <span v-if="loadingTables" class="text-xs font-medium text-blue-700">
          กำลังโหลดรายการ Table...
        </span>
      </section>

      <div class="grid grid-cols-1 gap-4 lg:grid-cols-12">
        <div class="lg:col-span-5">
          <SchemaExplorer
            :config="config"
            :tables="tables"
            :refresh-key="refreshKey"
            v-model:selected-columns="selectedColumns"
            @source-changed="(name) => (suggestedClassName = name)"
            @error="(m) => showToast('error', m)"
          />
        </div>

        <div class="lg:col-span-7">
          <CodegenPanel
            :columns="selectedColumns"
            :suggested-class-name="suggestedClassName"
            @error="(m) => showToast('error', m)"
            @saved="(m) => showToast('success', m)"
          />
        </div>
      </div>
    </div>
  </div>
</template>