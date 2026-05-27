// TauriStorage — Tauri invoke-based storage for Android local APK mode.
// Replaces NodeStorage's HTTP fetch calls with direct Tauri command invocations.
// Binary data is base64-encoded on the wire (Tauri string commands).

import { decodeRisuSave, encodeRisuSaveLegacy } from "./risuSave"
import { normalizeChat } from "./database.svelte"
import type { PatchItemResult } from "./nodeStorage"

// Lazy-loaded invoke to avoid import errors when Tauri is not available.
let _invoke: typeof import('@tauri-apps/api/core').invoke | null = null

async function getInvoke() {
    if (!_invoke) {
        const mod = await import('@tauri-apps/api/core')
        _invoke = mod.invoke
    }
    return _invoke
}

function uint8ArrayToBase64(data: Uint8Array): string {
    return Buffer.from(data).toString('base64')
}

function base64ToBuffer(b64: string): Buffer {
    return Buffer.from(b64, 'base64')
}

export class TauriStorage {
    _lastDbEtag: string | null = null
    authChecked = true  // No auth needed for local APK

    async createAuth(): Promise<string> {
        // No JWT needed for local Tauri app
        return ''
    }

    setDbEtag(etag: string | null) {
        this._lastDbEtag = etag
    }

    async setItem(key: string, value: Uint8Array, _etag?: string): Promise<void> {
        const invoke = await getInvoke()
        const data = uint8ArrayToBase64(value)
        await invoke('kv_write', { file_path: key, data })
    }

    async getItem(key: string): Promise<Buffer | null> {
        const invoke = await getInvoke()
        const result: string | null = await invoke('kv_read', { file_path: key })
        if (result === null || result === undefined) {
            return null
        }
        return base64ToBuffer(result)
    }

    async keys(prefix: string = ''): Promise<string[]> {
        const invoke = await getInvoke()
        const result: string[] = await invoke('kv_list', { key_prefix: prefix || undefined })
        return result
    }

    async removeItem(key: string): Promise<void> {
        const invoke = await getInvoke()
        await invoke('kv_remove', { file_path: key })
    }

    async patchItem(key: string, patchData: { patch: any[], expectedHash: string }): Promise<PatchItemResult> {
        // Read-modify-write: read current value, apply patch, write back.
        // This is a simplified implementation — no server-side atomicity.
        try {
            const { applyPatch } = await import('fast-json-patch')
            const current = await this.getItem(key)
            if (!current || current.length === 0) {
                return { success: false }
            }
            const decoded = await decodeRisuSave(new Uint8Array(current))
            const patched = applyPatch(decoded, patchData.patch, true, false).newDocument
            const encoded = encodeRisuSaveLegacy(patched)
            await this.setItem(key, new Uint8Array(encoded))
            return { success: true }
        } catch (error) {
            console.error('TauriStorage.patchItem failed:', error)
            return { success: false }
        }
    }

    // ── Bulk asset operations ──────────────────────────────────────────────────

    async getItems(keys: string[]): Promise<{ key: string, value: Buffer }[]> {
        const invoke = await getInvoke()
        const results: (string | null)[] = await invoke('assets_bulk_read', { keys })
        return results
            .map((val, i) => ({
                key: keys[i],
                value: val !== null ? base64ToBuffer(val) : null
            }))
            .filter((item): item is { key: string, value: Buffer } => item.value !== null)
    }

    async setItems(entries: { key: string, value: Uint8Array }[]): Promise<void> {
        const invoke = await getInvoke()
        const items = entries.map(e => ({
            key: e.key,
            value: uint8ArrayToBase64(e.value)
        }))
        await invoke('assets_bulk_write', { items })
    }

    // ── Chat content (runtime lazy load) ────────────────────────────────────

    async fetchChatContent(chaId: string, chatIndex: number, _chatId: string): Promise<any | null> {
        const invoke = await getInvoke()
        const result: string | null = await invoke('chat_content_load', {
            cha_id: chaId,
            chat_index: chatIndex.toString()
        })
        if (result === null || result === undefined) {
            return null
        }
        const buffer = base64ToBuffer(result)
        return normalizeChat(await decodeRisuSave(new Uint8Array(buffer)))
    }

    async saveChatContent(chaId: string, chatIndex: number, _chatId: string, chat: any): Promise<void> {
        const invoke = await getInvoke()
        const encoded = encodeRisuSaveLegacy(chat)
        const data = uint8ArrayToBase64(new Uint8Array(encoded))
        await invoke('chat_content_save', {
            cha_id: chaId,
            chat_index: chatIndex.toString(),
            data
        })
    }

    // ── Backup stubs ──────────────────────────────────────────────────────────

    async exportBackup(_opts?: { target?: 'upstream' }): Promise<Response> {
        throw new Error('Backup export is not yet supported in local APK mode')
    }

    async importBackup(
        _file: Blob,
        _onProgress?: (loaded: number, total: number) => void
    ): Promise<{ ok: boolean, assetsRestored: number, coldStorageFailed?: number }> {
        throw new Error('Backup import is not yet supported in local APK mode')
    }

    // ── Server-side backup stubs ──────────────────────────────────────────────

    async saveServerBackup(
        _onProgress?: (current: number, total: number, bytes: number, totalBytes: number) => void
    ): Promise<{ ok: boolean, filename: string, size: number }> {
        throw new Error('Server backup is not available in local APK mode')
    }

    async listServerBackups(): Promise<{ backups: Array<{ filename: string, size: number, createdAt: number }> }> {
        return { backups: [] }
    }

    async restoreServerBackup(
        _filename: string,
        _onProgress?: (bytes: number, totalBytes: number) => void
    ): Promise<{ ok: boolean, assetsRestored: number, coldStorageFailed?: number }> {
        throw new Error('Server backup restore is not available in local APK mode')
    }

    async deleteServerBackup(_filename: string): Promise<void> {
        throw new Error('Server backup delete is not available in local APK mode')
    }

    async downloadServerBackup(_filename: string): Promise<Response> {
        throw new Error('Server backup download is not available in local APK mode')
    }

    // ── Save-folder migration stubs ──────────────────────────────────────────

    async scanSaveFolder(_folderPath?: string): Promise<{ count: number, totalSize: number, hasDatabase: boolean }> {
        return { count: 0, totalSize: 0, hasDatabase: false }
    }

    async executeSaveFolderImport(_folderPath?: string): Promise<{ ok: boolean, imported: number }> {
        throw new Error('Save folder import is not available in local APK mode')
    }

    async uploadSaveFolderZip(
        _file: Blob,
        _onProgress?: (loaded: number, total: number) => void
    ): Promise<{ ok: boolean, imported: number }> {
        throw new Error('Save folder zip upload is not available in local APK mode')
    }

    async scanCleanup(): Promise<{ count: number, totalSize: number }> {
        return { count: 0, totalSize: 0 }
    }

    async executeCleanup(): Promise<{ ok: boolean, removed: number, freedBytes: number }> {
        return { ok: true, removed: 0, freedBytes: 0 }
    }

    listItem = this.keys
}
