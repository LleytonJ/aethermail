/**
 * Thread service - provides API calls for email threading functionality
 */

import { invoke } from '@tauri-apps/api/core'

export interface Thread {
  id: string;
  accountId: string;
  subjectBase: string | null;
  lastMessageAt: number;
  messageCount: number;
  hasUnread: boolean;
}

export interface ThreadAssignment {
  emailId: string;
  threadId: string;
}

export interface ThreadEmailInfo {
  id: string;
  accountId: string;
  fromAddress: string | null;
  fromName: string | null;
  toAddresses: string[];
  ccAddresses: string[];
  subject: string | null;
  snippet: string;
  date: number;
  isRead: boolean;
  isStarred: boolean;
  hasAttachment: boolean;
  isPhishing: boolean;
  phishingReasons: string[] | null;
  messageIdHeader: string | null;
  inReplyToHeader: string | null;
  referencesHeader: string | null;
  threadId: string | null;
}

/**
 * Get all threads for an account
 */
export async function getThreads(accountId: string): Promise<Thread[]> {
  return invoke<Thread[]>('get_threads', { accountId });
}

/**
 * Get all emails in a thread
 */
export async function getThreadEmails(threadId: string): Promise<ThreadEmailInfo[]> {
  return invoke<ThreadEmailInfo[]>('get_thread_emails', { threadId });
}

/**
 * Assign thread IDs to emails (called after computing threads)
 */
export async function assignThreads(assignments: ThreadAssignment[]): Promise<void> {
  return invoke<void>('assign_threads', { assignments });
}

/**
 * Get emails with threading headers for computation
 */
export async function getEmailsForThreading(accountId: string): Promise<ThreadEmailInfo[]> {
  return invoke<ThreadEmailInfo[]>('get_emails_for_threading', { accountId });
}

export { type ThreadableMessage, type ThreadGroup, buildThreads, updateThreads } from './threadBuilder';