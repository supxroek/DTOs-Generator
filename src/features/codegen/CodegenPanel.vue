<script setup lang="ts">
import { ref, watch } from 'vue';
import { open } from '@tauri-apps/plugin-dialog';
import { tauriApi } from '../../shared/api/tauriApi';
import type {
  CodegenOptions,
  ColumnSpec,
  GeneratedCodeOutput,
} from '../../shared/types/dtoGen';

const props = defineProps<{
  columns: ColumnSpec[];
  suggestedClassName: string;
}>();

const emit = defineEmits<{
  error: [message: string];
  saved: [message: string];
}>();

const options = ref<CodegenOptions>({
  className: 'UserDto',
  namespace: 'MyWinFormsApp.DTOs',
  target: 'netFramework4x',
  tableVarName: 'dt',
  rowVarName: 'row',
});

const output = ref<GeneratedCodeOutput>({
  dtoCode: '',
  mapperCode: '',
});

const activeTab = ref<'dto' | 'mapper'>('dto');
const targetFolder = ref('');
const copied = ref<'dto' | 'mapper' | null>(null);
const saving = ref(false);

watch(
  () => props.suggestedClassName,
  (name) => {
    if (name) {
      options.value.className = name;
    }
  }
);

async function regenerate() {
  if (props.columns.length === 0) {
    output.value = { dtoCode: '', mapperCode: '' };
    return;
  }
  try {
    output.value = await tauriApi.generateDto(props.columns, options.value);
  } catch (err: any) {
    emit('error', err?.message ?? String(err));
  }
}

watch(
  [() => props.columns, options],
  () => {
    regenerate();
  },
  { deep: true, immediate: true }
);

async function copyCode(kind: 'dto' | 'mapper') {
  const text = kind === 'dto' ? output.value.dtoCode : output.value.mapperCode;
  if (!text) return;
  await navigator.clipboard.writeText(text);
  copied.value = kind;
  setTimeout(() => {
    if (copied.value === kind) copied.value = null;
  }, 1500);
}

async function pickFolder() {
  const selected = await open({
    directory: true,
    multiple: false,
    title: 'เลือกโฟลเดอร์ Models / DTOs ของโปรเจกต์ WinForms',
  });
  if (typeof selected === 'string') {
    targetFolder.value = selected;
  }
}

async function saveDtoFile() {
  if (!targetFolder.value || !output.value.dtoCode) return;
  saving.value = true;
  try {
    const info = await tauriApi.saveCsFile({
      folderPath: targetFolder.value,
      fileName: options.value.className,
      content: output.value.dtoCode,
      overwrite: true,
    });
    emit('saved', `บันทึกไฟล์เรียบร้อย: ${info.fullPath}`);
  } catch (err: any) {
    emit('error', err?.message ?? String(err));
  } finally {
    saving.value = false;
  }
}
</script>

<template>
  <section class="flex h-full flex-col rounded-xl border border-slate-200 bg-white p-4 shadow-sm">
    <!-- ตัวเลือก Codegen -->
    <div class="grid grid-cols-1 gap-3 sm:grid-cols-2 lg:grid-cols-4">
      <div>
        <label class="mb-1 block text-xs font-medium text-slate-600">Class Name</label>
        <input
          v-model="options.className"
          type="text"
          class="w-full rounded-lg border border-slate-300 px-2.5 py-1.5 text-sm focus:border-blue-500 focus:outline-none"
        />
      </div>

      <div>
        <label class="mb-1 block text-xs font-medium text-slate-600">Namespace</label>
        <input
          v-model="options.namespace"
          type="text"
          class="w-full rounded-lg border border-slate-300 px-2.5 py-1.5 text-sm focus:border-blue-500 focus:outline-none"
        />
      </div>

      <div>
        <label class="mb-1 block text-xs font-medium text-slate-600">Target Framework</label>
        <select
          v-model="options.target"
          class="w-full rounded-lg border border-slate-300 px-2.5 py-1.5 text-sm focus:border-blue-500 focus:outline-none"
        >
          <option value="netFramework4x">.NET Framework 4.x (WinForms)</option>
          <option value="netModern">.NET 8+ (WinForms)</option>
        </select>
      </div>

      <div class="grid grid-cols-2 gap-2">
        <div>
          <label class="mb-1 block text-xs font-medium text-slate-600">DataTable Var</label>
          <input
            v-model="options.tableVarName"
            type="text"
            class="w-full rounded-lg border border-slate-300 px-2.5 py-1.5 font-mono text-sm focus:border-blue-500 focus:outline-none"
          />
        </div>
        <div>
          <label class="mb-1 block text-xs font-medium text-slate-600">DataRow Var</label>
          <input
            v-model="options.rowVarName"
            type="text"
            class="w-full rounded-lg border border-slate-300 px-2.5 py-1.5 font-mono text-sm focus:border-blue-500 focus:outline-none"
          />
        </div>
      </div>
    </div>

    <!-- แท็บโค้ด 2 ส่วน + ปุ่ม Copy -->
    <div class="mt-4 flex flex-wrap items-center justify-between gap-2 border-b border-slate-200 pb-2">
      <div class="inline-flex rounded-lg bg-slate-100 p-1 text-xs font-medium">
        <button
          type="button"
          @click="activeTab = 'dto'"
          :class="
            activeTab === 'dto'
              ? 'rounded-md bg-white px-3 py-1 text-slate-900 shadow-sm'
              : 'px-3 py-1 text-slate-600 hover:text-slate-900'
          "
        >
          1. Class DTO (.cs)
        </button>
        <button
          type="button"
          @click="activeTab = 'mapper'"
          :class="
            activeTab === 'mapper'
              ? 'rounded-md bg-white px-3 py-1 text-slate-900 shadow-sm'
              : 'px-3 py-1 text-slate-600 hover:text-slate-900'
          "
        >
          2. DataRow Loop
        </button>
      </div>

      <button
        type="button"
        @click="copyCode(activeTab)"
        class="rounded-lg border border-slate-300 px-3 py-1 text-xs font-medium text-slate-700 hover:bg-slate-50"
      >
        {{ copied === activeTab ? 'Copied!' : 'Copy Code' }}
      </button>
    </div>

    <!-- พื้นที่แสดงโค้ด -->
    <pre
      class="mt-3 min-h-65 flex-1 overflow-auto rounded-lg bg-slate-900 p-3.5 font-mono text-xs leading-relaxed text-slate-100"
    ><code>{{ activeTab === 'dto' ? output.dtoCode || '// เลือก Table หรือวิเคราะห์ SQL Query เพื่อสร้าง DTO' : output.mapperCode || '// เลือก Table หรือวิเคราะห์ SQL Query เพื่อสร้าง DataRow Loop' }}</code></pre>

    <!-- บันทึกไฟล์ .cs ลงโฟลเดอร์ -->
    <div class="mt-3 flex flex-wrap items-center gap-2 border-t border-slate-100 pt-3">
      <input
        v-model="targetFolder"
        type="text"
        readonly
        placeholder="ยังไม่ได้เลือกโฟลเดอร์ปลายทางของ WinForms..."
        class="min-w-55 flex-1 rounded-lg border border-slate-200 bg-slate-50 px-3 py-1.5 text-xs text-slate-600"
      />
      <button
        type="button"
        @click="pickFolder"
        class="rounded-lg border border-slate-300 px-3 py-1.5 text-xs font-medium text-slate-700 hover:bg-slate-50"
      >
        เลือกโฟลเดอร์...
      </button>
      <button
        type="button"
        :disabled="saving || !targetFolder || !output.dtoCode"
        @click="saveDtoFile"
        class="rounded-lg bg-emerald-600 px-4 py-1.5 text-xs font-medium text-white hover:bg-emerald-700 disabled:opacity-50"
      >
        {{ saving ? 'กำลังบันทึก...' : 'บันทึกไฟล์ .cs' }}
      </button>
    </div>
  </section>
</template>