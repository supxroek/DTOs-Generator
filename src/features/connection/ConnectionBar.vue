<script setup lang="ts">
import { ref } from 'vue';
import { tauriApi } from '../../shared/api/tauriApi';
import type { ConnectionConfig } from '../../shared/types/dtoGen';

const config = defineModel<ConnectionConfig>('config', { required: true });

const emit = defineEmits<{
  connected: [];
  databasesLoaded: [databases: string[]];
  error: [message: string];
}>();

const loading = ref(false);
const statusText = ref('');

async function handleConnect() {
  loading.value = true;
  statusText.value = '';
  try {
    const dbName = config.value.database.trim();

    if (!dbName) {
      // กรณีไม่ระบุ Database: ดึงรายชื่อฐานข้อมูลทั้งหมดมาให้เลือกก่อน
      const dbs = await tauriApi.listDatabases(config.value);
      statusText.value = `พบ ${dbs.length} ฐานข้อมูล กรุณาเลือกฐานข้อมูลด้านล่าง`;
      emit('databasesLoaded', dbs);
    } else {
      // กรณีระบุ Database แล้ว: ทดสอบเชื่อมต่อและโหลด Tables ทันที
      await tauriApi.testConnection(config.value);
      statusText.value = `เชื่อมต่อฐานข้อมูล ${dbName} สำเร็จ`;
      emit('connected');
    }
  } catch (err: any) {
    const msg = err?.message ?? String(err);
    statusText.value = `เชื่อมต่อไม่สำเร็จ: ${msg}`;
    emit('error', msg);
  } finally {
    loading.value = false;
  }
}
</script>

<template>
  <section class="rounded-xl border border-slate-200 bg-white p-4 shadow-sm">
    <div class="grid grid-cols-1 gap-3 md:grid-cols-6">
      <div class="md:col-span-2">
        <label class="mb-1 block text-xs font-medium text-slate-600">Host / Server</label>
        <input
          v-model="config.host"
          type="text"
          placeholder="localhost หรือ 127.0.0.1"
          class="w-full rounded-lg border border-slate-300 px-3 py-1.5 text-sm focus:border-blue-500 focus:outline-none"
        />
      </div>

      <div>
        <label class="mb-1 block text-xs font-medium text-slate-600">Port</label>
        <input
          v-model.number="config.port"
          type="number"
          class="w-full rounded-lg border border-slate-300 px-3 py-1.5 text-sm focus:border-blue-500 focus:outline-none"
        />
      </div>

      <div>
        <label class="mb-1 block text-xs font-medium text-slate-600">Database</label>
        <input
          v-model="config.database"
          type="text"
          placeholder="เว้นว่างเพื่อดึงรายการ DB"
          class="w-full rounded-lg border border-slate-300 px-3 py-1.5 text-sm focus:border-blue-500 focus:outline-none"
        />
      </div>

      <div>
        <label class="mb-1 block text-xs font-medium text-slate-600">Username</label>
        <input
          v-model="config.username"
          type="text"
          placeholder="sa"
          class="w-full rounded-lg border border-slate-300 px-3 py-1.5 text-sm focus:border-blue-500 focus:outline-none"
        />
      </div>

      <div>
        <label class="mb-1 block text-xs font-medium text-slate-600">Password</label>
        <input
          v-model="config.password"
          type="password"
          placeholder="••••••••"
          class="w-full rounded-lg border border-slate-300 px-3 py-1.5 text-sm focus:border-blue-500 focus:outline-none"
        />
      </div>
    </div>

    <div class="mt-3 flex flex-wrap items-center justify-between gap-3">
      <label class="flex items-center gap-2 text-xs text-slate-600">
        <input v-model="config.trustCert" type="checkbox" class="rounded border-slate-300" />
        Trust Server Certificate (แนะนำสำหรับ Dev/LAN)
      </label>

      <div class="flex items-center gap-3">
        <span v-if="statusText" class="text-xs font-medium text-slate-700">{{ statusText }}</span>
        <button
          :disabled="loading"
          type="button"
          @click="handleConnect"
          class="rounded-lg bg-blue-600 px-4 py-1.5 text-sm font-medium text-white hover:bg-blue-700 disabled:opacity-50"
        >
          {{
            loading
              ? 'กำลังเชื่อมต่อ...'
              : config.database.trim()
                ? 'Test & Load Tables'
                : 'Connect & List Databases'
          }}
        </button>
      </div>
    </div>
  </section>
</template>