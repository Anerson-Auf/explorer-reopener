# Explorer Reopener

Я сделал эту небольшую Windows-утилиту после того, как некоторые игры начали время от времени ронять `explorer.exe` на моём компьютере. Точную причину я пока не выяснил: это может быть нехватка оперативной памяти, конфликт с игрой или другая проблема Windows.

Чтобы не запускать Explorer вручную после каждого такого сбоя, я сделал простой watchdog. Он работает в фоне и автоматически запускает `explorer.exe`, если процесс исчез.

## Что делает программа

- проверяет наличие `explorer.exe` раз в 10 секунд;
- запускает его, если процесс отсутствует;
- при первом запуске создаёт ярлык самой себя в папке автозагрузки Windows;
- работает без консольного окна и внешних сервисов.

Программа не пытается определить, почему Explorer завис или упал. Она только возвращает процесс, если его больше нет.

## Требования

- Windows;
- Rust stable;
- MSVC toolchain для Windows.

## Сборка

```powershell
cargo build --release
```

Готовый файл появится здесь:

```text
target\release\explorerreop.exe
```

Запуск:

```powershell
.\target\release\explorerreop.exe
```

При первом запуске ярлык создаётся в:

```text
%APPDATA%\Microsoft\Windows\Start Menu\Programs\Startup
```

## Удаление из автозагрузки

Закройте программу и удалите файл:

```text
%APPDATA%\Microsoft\Windows\Start Menu\Programs\Startup\rexplorer.lnk
```

## GitHub Actions

Workflow в `.github/workflows/build.yml` собирает release-версию на Windows при каждом `push`, pull request или ручном запуске. Готовый `.exe` доступен в разделе **Actions → Artifacts** соответствующего запуска.

## Статус

Проект находится на ранней стадии. Сейчас это простой watchdog для процесса Explorer, а не полноценный мониторинг зависших или повреждённых компонентов Windows.
