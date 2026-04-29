export const mergeSorted = <T>(
  current: T[],
  updated: T[],
  comparer: (a: T, b: T) => number,
  matcher: (a: T, b: T) => boolean,
): T[] => {
  const merged: T[] = []
  let currentIndex = 0
  let updatedIndex = 0
  let changed = false

  while (currentIndex < current.length && updatedIndex < updated.length) {
    const currentItem = current[currentIndex]
    const updatedItem = updated[updatedIndex]
    const cmp = comparer(currentItem, updatedItem)
    if (cmp < 0) {
      ++currentIndex // Gone
      changed = true
    } else if (cmp > 0) {
      merged.push(updatedItem)
      ++updatedIndex // Added
      changed = true
    } else {
      if (matcher(currentItem, updatedItem)) {
        merged.push(currentItem)
      } else {
        merged.push(updatedItem)
        changed = true
        // Change callback?
      }

      ++currentIndex
      ++updatedIndex
    }
  }

  while (updatedIndex < updated.length) {
    merged.push(updated[updatedIndex])
    ++updatedIndex
    changed = true
  }

  if (currentIndex < current.length) {
    changed = true // Remaining current items dropped
  }

  return changed ? merged : current
}
