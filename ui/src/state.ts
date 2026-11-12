import { createContext, useContext } from 'react'
import type { Assessment, HistoryEntry, ProxyStatus, UiFinding, View } from './lib/types'

export interface AppState {
  assessments: Assessment[]
  selected: Assessment | null
  select: (id: string | null) => void
  refreshAssessments: () => Promise<void>
  openNewProject: () => void
  projectsLoaded: boolean
  projectsError: string | null

  findings: UiFinding[]
  findingsLoading: boolean
  findingsError: string | null
  refreshFindings: () => Promise<void>
  focusFinding: string | null
  setFocusFinding: (id: string | null) => void

  proxy: ProxyStatus | null
  setProxy: (p: ProxyStatus) => void
  traffic: HistoryEntry[]
  freshIds: Set<string>
  trafficError: string | null
  refreshTraffic: () => Promise<void>

  repeaterReq: string
  setRepeaterReq: (s: string) => void
  toRepeater: (raw: string) => void

  view: View
  go: (v: View) => void
  fail: (e: unknown) => void
}

export const AppContext = createContext<AppState | null>(null)

export function useApp(): AppState {
  const ctx = useContext(AppContext)
  if (!ctx) throw new Error('useApp outside AppContext')
  return ctx
}
