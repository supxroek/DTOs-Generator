<script setup lang="ts">
import { ref, watch } from 'vue';
import { tauriApi } from '../../shared/api/tauriApi';
import type { ColumnSpec, ConnectionConfig, TableInfo } from '../../shared/types/dtoGen';

export interface SelectableColumn {
  selected: boolean;
  spec: ColumnSpec;
}

const props = defineProps<{
  config: ConnectionConfig;
  tables: TableInfo[];
  refreshKey: number;
}>();

const selectedColumns = defineModel<ColumnSpec[]>('selectedColumns', { required: true });

const emit = defineEmits<{
  sourceChanged: [suggestedName: string];
  error: [message: string];
}>();

const mode = ref<'table' | 'query'>('table');
const chosenTableKey = ref('');
const sqlQuery = ref(
  'SELECT TOP 10 u.Id, u.UserName, u.Email, o.TotalAmount\nFROM Users u\nLEFT JOIN Orders o ON o.UserId = u.Id'
);
const items = ref<SelectableColumn[]>([]);
const loading = ref(false);

function syncSelected() {
  selectedColumns.value = items.value.filter((x) => x.selected).map((x) => ({ ...x.spec }));
}

function setLoadedColumns(cols: ColumnSpec[], suggestedName: string) {
  items.value = cols.map((spec) => ({
    selected: true,
    spec: { ...spec },
  }));
  syncSelected();
  emit('sourceChanged', suggestedName);
}

async function handleLoadTable() {
  if (!chosenTableKey.value) return;
  const [schema, name] = chosenTableKey.value.split('::');
  if (!schema || !name) return;

  loading.value = true;
  try {
    const cols = await tauriApi.describeTable(props.config, schema, name);
    setLoadedColumns(cols, `${name}Dto`);
  } catch (err: any) {
    emit('error', err?.message ?? String(err));
  } finally {
    loading.value = false;
  }
}

async function handleAnalyzeQuery() {
  if (!sqlQuery.value.trim()) return;
  loading.value = true;
  try {
    const cols = await tauriApi.describeQuery(props.config, sqlQuery.value);
    setLoadedColumns(cols, 'QueryResultDto');
  } catch (err: any) {
    emit('error', err?.message ?? String(err));
  } finally {
    loading.value = false;
  }
}

watch(
  () => props.tables,
  (list) => {
    if (list.length > 0 && !chosenTableKey.value) {
      chosenTableKey.value = `${list[0].schema}::${list[0].name}`;
    }
  }
);

function formatSqlType(sqlType: ColumnSpec['sqlType']): string {
  return typeof sqlType === 'string' ? sqlType : sqlType.other;
}
</script>

<template>
  <section class="rounded-xl border border-slate-200 bg-white p-4 shadow-sm">
    <div class="mb-3 flex items-center justify-between">
      <div class="inline-flex rounded-lg bg-slate-100 p-1 text-xs font-medium">
        <button
          type="button"
          @click="mode = 'table'"
          :class="
            mode === 'table'
              ? 'rounded-md bg-white px-3 py-1 text-slate-900 shadow-sm'
              : 'px-3 py-1 text-slate-600 hover:text-slate-900'
          "
        >
          Table / View
        </button>
        <button
          type="button"
          @click="mode = 'query'"
          :class="
            mode === 'query'
              ? 'rounded-md bg-white px-3 py-1 text-slate-900 shadow-sm'
              : 'px-3 py-1 text-slate-600 hover:text-slate-900'
          "
        >
          SQL Query
        </button>
      </div>

      <span class="text-xs text-slate-500">
        เลือกแล้ว {{ selectedColumns.length }} / {{ items.length }} คอลัมน์
      </span>
    </div>

    <!-- โหมด Table / View -->
    <div v-if="mode === 'table'" class="flex flex-wrap items-center gap-2">
      <select
        v-model="chosenTableKey"
        class="min-w-60 flex-1 rounded-lg border border-slate-300 px-3 py-1.5 text-sm focus:border-blue-500 focus:outline-none"
      >
        <option value="" disabled>-- เลือก Table หรือ View --</option>
        <option v-for="t in tables" :key="`${t.schema}.${t.name}`" :value="`${t.schema}::${t.name}`">
          {{ t.schema }}.{{ t.name }}
        </option>
      </select>

      <button
        type="button"
        :disabled="loading || !chosenTableKey"
        @click="handleLoadTable"
        class="rounded-lg bg-slate-800 px-4 py-1.5 text-sm font-medium text-white hover:bg-slate-900 disabled:opacity-50"
      >
        {{ loading ? 'กำลังอ่าน...' : 'ดึงคอลัมน์' }}
      </button>
    </div>

    <!-- โหมด SQL Query -->
    <div v-else class="space-y-2">
      <textarea
        v-model="sqlQuery"
        rows="12"
        placeholder="วางคำสั่ง SELECT ... JOIN ..."
        class="w-full rounded-lg border border-slate-300 p-2.5 font-mono text-xs focus:border-blue-500 focus:outline-none"
      />
      <div class="flex justify-end">
        <button
          type="button"
          :disabled="loading || !sqlQuery.trim()"
          @click="handleAnalyzeQuery"
          class="rounded-lg bg-slate-800 px-4 py-1.5 text-sm font-medium text-white hover:bg-slate-900 disabled:opacity-50"
        >
          {{ loading ? 'กำลังวิเคราะห์...' : 'วิเคราะห์ผลลัพธ์ Query' }}
        </button>
      </div>
    </div>

    <!-- ตารางรายการคอลัมน์ -->
    <div v-if="items.length > 0" class="mt-4 max-h-72 overflow-auto rounded-lg border border-slate-200">
      <table class="w-full text-left text-xs">
        <thead class="sticky top-0 bg-slate-50 text-slate-600">
          <tr>
            <th class="w-10 px-3 py-2">
              <input
                type="checkbox"
                :checked="items.length > 0 && items.every((i) => i.selected)"
                @change="
                  const next = ($event.target as HTMLInputElement).checked;
                  items.forEach((i) => (i.selected = next));
                  syncSelected();
                "
              />
            </th>
            <th class="px-3 py-2">Column Name</th>
            <th class="px-3 py-2">SQL Type</th>
            <th class="px-3 py-2">Nullable</th>
          </tr>
        </thead>
        <tbody class="divide-y divide-slate-100">
          <tr v-for="col in items" :key="col.spec.rawName" class="hover:bg-slate-50/70">
            <td class="px-3 py-1.5">
              <input
                v-model="col.selected"
                type="checkbox"
                @change="syncSelected"
              />
            </td>
            <td class="px-3 py-1.5 font-mono text-slate-800">{{ col.spec.rawName }}</td>
            <td class="px-3 py-1.5 font-mono text-slate-600">{{ formatSqlType(col.spec.sqlType) }}</td>
            <td class="px-3 py-1.5">
              <input
                v-model="col.spec.isNullable"
                type="checkbox"
                @change="syncSelected"
              />
            </td>
          </tr>
        </tbody>
      </table>
    </div>
  </section>
</template>