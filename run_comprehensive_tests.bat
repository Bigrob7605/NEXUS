@echo off
REM Comprehensive Compression Test Runner for Windows
REM Tests the fixed compression system across 10 programming languages with real codebases

echo.
echo ================================================================================
echo 🧪 COMPREHENSIVE COMPRESSION TEST RUNNER
echo Testing the fixed compression system across 10 programming languages
echo ================================================================================
echo.

REM Check if Python is available
python --version >nul 2>&1
if errorlevel 1 (
    echo ❌ Python is not available. Please install Python 3.7+ and try again.
    pause
    exit /b 1
)

REM Check if git is available
git --version >nul 2>&1
if errorlevel 1 (
    echo ❌ Git is not available. Please install Git and try again.
    pause
    exit /b 1
)

echo ✅ Prerequisites check passed
echo.

REM Check command line arguments
if "%1"=="--skip-download" (
    echo 📥 Skipping codebase download (using existing codebases)
    echo.
    python tests/run_comprehensive_compression_tests.py --skip-download
) else if "%1"=="--download-only" (
    echo 📥 Running codebase download only
    echo.
    python tests/download_real_codebases.py
) else if "%1"=="--multi-language" (
    echo 🌍 Running multi-language compression tests only
    echo.
    python tests/run_comprehensive_compression_tests.py --phase multi_language
) else if "%1"=="--real-integration" (
    echo 🔧 Running real compression integration tests only
    echo.
    python tests/run_comprehensive_compression_tests.py --phase real_integration
) else if "%1"=="--help" (
    echo Usage options:
    echo   run_comprehensive_tests.bat              - Run all tests (download + testing)
    echo   run_comprehensive_tests.bat --skip-download  - Skip download, use existing codebases
    echo   run_comprehensive_tests.bat --download-only  - Download codebases only
    echo   run_comprehensive_tests.bat --multi-language - Run multi-language tests only
    echo   run_comprehensive_tests.bat --real-integration - Run real integration tests only
    echo   run_comprehensive_tests.bat --help       - Show this help message
    echo.
    echo Note: First run will download ~40 real codebases (may take time)
    echo.
) else (
    echo 🚀 Running comprehensive compression tests across all phases
    echo.
    echo This will:
    echo   1. Download real codebases from 10 programming languages
    echo   2. Run multi-language compression tests
    echo   3. Run real compression integration tests
    echo.
    echo First run may take 10-30 minutes to download codebases
    echo.
    python tests/run_comprehensive_compression_tests.py
)

echo.
echo ================================================================================
echo Test execution completed
echo ================================================================================
echo.
echo 📄 Reports generated:
echo   - comprehensive_test_report.md
echo   - comprehensive_test_results.json
echo   - test_report.md (multi-language)
echo   - real_compression_test_report.md (real integration)
echo   - codebase_download_report.md (download)
echo.
pause
