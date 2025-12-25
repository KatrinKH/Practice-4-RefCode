<?php

namespace App\Http\Controllers;

use App\Services\IssService;

class IssController extends Controller
{
    public function index(IssService $service)
    {
        $last  = $service->getLast();
        $trend = $service->getTrend();

        return view('iss', [
            'last'  => $last,
            'trend' => $trend,
            'base'  => config('services.rust.url'),
        ]);
    }
}
