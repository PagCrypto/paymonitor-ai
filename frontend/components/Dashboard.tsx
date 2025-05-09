import { useEffect, useState } from 'react'
import { fetchRiskData } from '../lib/api'

export default function Dashboard() {
  const [data, setData] = useState<any[]>([])

  useEffect(() => {
    fetchRiskData().then(setData)
  }, [])

  return (
    <div className="grid gap-4 mt-4">
      <div className="bg-gray-900 text-white p-4 rounded-xl shadow-xl">
        <h2 className="text-xl font-semibold mb-2">Resumo de Risco</h2>
        <pre className="whitespace-pre-wrap text-sm">
          {JSON.stringify(data, null, 2)}
        </pre>
      </div>
    </div>
  )
}