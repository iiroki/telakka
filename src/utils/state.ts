// TODO: These should be received from the Tauri backend!
export type ContainerState = 'unknown' | 'running' | 'exited' | 'paused' | 'restarting' | 'removing' | 'dead'
export type ContanerGroupState = 'unknown' | 'running' | 'partial' | 'exited'

export const calculateGroupState = (...states: ContainerState[]): ContanerGroupState => {
  if (states.length === 0) {
    return 'unknown'
  }

  let running = 0
  let unknown = 0
  for (const s of states) {
    if (s === 'running' || s === 'restarting') running++
    else if (s === 'unknown') unknown++
  }

  if (running === states.length) return 'running'
  if (running > 0) return 'partial'
  if (unknown === states.length) return 'unknown'
  return 'exited'
}
