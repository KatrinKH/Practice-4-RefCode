@extends('layout')

@section('title', $title ?? 'CMS Страница')

@section('content')
<div class="container my-3">
  {{-- Заголовок с экранированием --}}
  <h3 class="mb-3">{{ $title ?? 'Страница' }}</h3>
  
  {{-- Контент с обработкой --}}
  @php
      // Очистка от потенциально опасных тегов (базовый вариант)
      $safeContent = $html ?? '';
      // Удаляем script теги
      $safeContent = preg_replace('/<script\b[^>]*>(.*?)<\/script>/is', '', $safeContent);
      // Удаляем on* атрибуты
      $safeContent = preg_replace('/\s+on\w+\s*=\s*["\'][^"\']*["\']/i', '', $safeContent);
  @endphp
  
  <div class="cms-content">
    {!! $safeContent !!}
  </div>
  
  {{-- Отладочная информация (только для разработки) --}}
  @if(app()->environment('local') && !empty($html))
    <div class="card mt-4">
      <div class="card-header bg-light">
        <small>Отладка: исходный HTML ({{ strlen($html) }} символов)</small>
      </div>
      <div class="card-body">
        <pre class="mb-0 small text-muted">{{ htmlspecialchars($html) }}</pre>
      </div>
    </div>
  @endif
</div>
@endsection