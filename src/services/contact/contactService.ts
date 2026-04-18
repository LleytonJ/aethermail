/**
 * 联系人服务层 - 调用 Tauri 后端命令
 */

import { invoke } from '@tauri-apps/api/core'

/**
 * 联系人数据类型
 */
export interface Contact {
  id: string
  email: string
  displayName: string | null
  avatarUrl: string | null
  frequency: number
  lastContactedAt: number | null
}

/**
 * 搜索联系人（用于自动补全）
 */
export async function searchContacts(query: string, limit: number = 10): Promise<Contact[]> {
  const result = await invoke<Contact[]>('search_contacts', { query, limit })
  return result.map(c => ({
    id: c.id,
    email: c.email,
    displayName: c.displayName,
    avatarUrl: c.avatarUrl,
    frequency: c.frequency,
    lastContactedAt: c.lastContactedAt
  }))
}

/**
 * 获取所有联系人
 */
export async function getAllContacts(): Promise<Contact[]> {
  const result = await invoke<Contact[]>('get_all_contacts')
  return result.map(c => ({
    id: c.id,
    email: c.email,
    displayName: c.displayName,
    avatarUrl: c.avatarUrl,
    frequency: c.frequency,
    lastContactedAt: c.lastContactedAt
  }))
}

/**
 * 根据邮箱获取联系人
 */
export async function getContactByEmail(email: string): Promise<Contact | null> {
  const result = await invoke<Contact | null>('get_contact_by_email', { email })
  if (!result) return null
  return {
    id: result.id,
    email: result.email,
    displayName: result.displayName,
    avatarUrl: result.avatarUrl,
    frequency: result.frequency,
    lastContactedAt: result.lastContactedAt
  }
}

/**
 * 更新联系人
 */
export async function updateContact(id: string, displayName: string | null, avatarUrl: string | null): Promise<void> {
  await invoke('update_contact', { id, displayName, avatarUrl })
}

/**
 * 删除联系人
 */
export async function deleteContact(id: string): Promise<void> {
  await invoke('delete_contact', { id })
}