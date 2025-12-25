<?php

namespace App\Services;

use Illuminate\Support\Facades\Http;

class IssService
{
    private string $baseUrl;

    public function __construct()
    {
        $this->baseUrl = config('services.rust.url');
    }

    public function getLast(): array
    {
        return $this->getJson('/last');
    }

    public function getTrend(): array
    {
        return $this->getJson('/iss/trend');
    }

    private function getJson(string $path): array
    {
        $response = Http::timeout(5)
            ->retry(3, 200)
            ->get($this->baseUrl . $path);

        if ($response->failed()) {
            return [];
        }

        return $response->json() ?? [];
    }
}
