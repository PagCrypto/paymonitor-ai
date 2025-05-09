export async function fetchRiskData() {
  const res = await fetch('/api/risk')
  return await res.json()
}