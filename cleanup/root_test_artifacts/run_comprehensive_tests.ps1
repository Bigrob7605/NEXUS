#!/usr/bin/env pwsh
<#
.SYNOPSIS
    Comprehensive Compression Test Runner for Windows PowerShell
    
.DESCRIPTION
    Tests the fixed compression system across 10 programming languages with real codebases.
    This script orchestrates the entire testing process including downloading real codebases
    and running comprehensive compression tests.
    
.PARAMETER SkipDownload
    Skip downloading codebases and use existing ones
    
.PARAMETER DownloadOnly
    Only download codebases, don't run tests
    
.PARAMETER Phase
    Run only a specific test phase (download, multi_language, real_integration)
    
.PARAMETER Help
    Show help information
    
.EXAMPLE
    .\run_comprehensive_tests.ps1
    Run all tests (download + testing)
    
.EXAMPLE
    .\run_comprehensive_tests.ps1 -SkipDownload
    Skip download, use existing codebases
    
.EXAMPLE
    .\run_comprehensive_tests.ps1 -DownloadOnly
    Download codebases only
    
.EXAMPLE
    .\run_comprehensive_tests.ps1 -Phase multi_language
    Run multi-language tests only
    
.NOTES
    First run will download ~40 real codebases and may take 10-30 minutes.
    Requires Python 3.7+ and Git to be installed.
#>

param(
    [switch]$SkipDownload,
    [switch]$DownloadOnly,
    [ValidateSet("download", "multi_language", "real_integration")]
    [string]$Phase,
    [switch]$Help
)

# Set error action preference
$ErrorActionPreference = "Stop"

# Function to write colored output
function Write-ColorOutput {
    param(
        [string]$Message,
        [string]$Color = "White"
    )
    Write-Host $Message -ForegroundColor $Color
}

# Function to check prerequisites
function Test-Prerequisites {
    Write-ColorOutput "🔍 Checking system prerequisites..." "Cyan"
    
    # Check Python
    try {
        $pythonVersion = python --version 2>&1
        if ($LASTEXITCODE -eq 0) {
            Write-ColorOutput "✅ Python available: $pythonVersion" "Green"
        } else {
            throw "Python check failed"
        }
    } catch {
        Write-ColorOutput "❌ Python is not available. Please install Python 3.7+ and try again." "Red"
        return $false
    }
    
    # Check Git
    try {
        $gitVersion = git --version 2>&1
        if ($LASTEXITCODE -eq 0) {
            Write-ColorOutput "✅ Git available: $gitVersion" "Green"
        } else {
            throw "Git check failed"
        }
    } catch {
        Write-ColorOutput "❌ Git is not available. Please install Git and try again." "Red"
        return $false
    }
    
    # Check required directories
    $requiredDirs = @("src", "tests", "logs")
    foreach ($dir in $requiredDirs) {
        if (-not (Test-Path $dir)) {
            Write-ColorOutput "❌ Required directory '$dir' not found" "Red"
            return $false
        }
    }
    
    Write-ColorOutput "✅ All prerequisites met" "Green"
    return $true
}

# Function to run Python script
function Invoke-PythonScript {
    param(
        [string]$ScriptPath,
        [string[]]$Arguments = @()
    )
    
    $allArgs = @($ScriptPath) + $Arguments
    Write-ColorOutput "🚀 Running: python $($allArgs -join ' ')" "Yellow"
    
    try {
        $result = & python $allArgs 2>&1
        $exitCode = $LASTEXITCODE
        
        if ($exitCode -eq 0) {
            Write-ColorOutput "✅ Script completed successfully" "Green"
            return @{ Success = $true; Output = $result; ExitCode = $exitCode }
        } else {
            Write-ColorOutput "❌ Script failed with exit code $exitCode" "Red"
            return @{ Success = $false; Output = $result; ExitCode = $exitCode }
        }
    } catch {
        Write-ColorOutput "❌ Script execution failed: $($_.Exception.Message)" "Red"
        return @{ Success = $false; Output = $null; ExitCode = -1; Error = $_.Exception.Message }
    }
}

# Main execution
function Main {
    Write-ColorOutput "" "White"
    Write-ColorOutput "================================================================================" "Cyan"
    Write-ColorOutput "🧪 COMPREHENSIVE COMPRESSION TEST RUNNER" "Cyan"
    Write-ColorOutput "Testing the fixed compression system across 10 programming languages" "Cyan"
    Write-ColorOutput "================================================================================" "Cyan"
    Write-ColorOutput "" "White"
    
    # Show help if requested
    if ($Help) {
        Write-ColorOutput "Usage options:" "Yellow"
        Write-ColorOutput "  .\run_comprehensive_tests.ps1                    - Run all tests (download + testing)" "White"
        Write-ColorOutput "  .\run_comprehensive_tests.ps1 -SkipDownload      - Skip download, use existing codebases" "White"
        Write-ColorOutput "  .\run_comprehensive_tests.ps1 -DownloadOnly      - Download codebases only" "White"
        Write-ColorOutput "  .\run_comprehensive_tests.ps1 -Phase multi_language - Run multi-language tests only" "White"
        Write-ColorOutput "  .\run_comprehensive_tests.ps1 -Phase real_integration - Run real integration tests only" "White"
        Write-ColorOutput "  .\run_comprehensive_tests.ps1 -Help               - Show this help message" "White"
        Write-ColorOutput "" "White"
        Write-ColorOutput "Note: First run will download ~40 real codebases (may take time)" "Yellow"
        Write-ColorOutput "" "White"
        return
    }
    
    # Check prerequisites
    if (-not (Test-Prerequisites)) {
        Write-ColorOutput "❌ Prerequisites not met. Cannot continue." "Red"
        Read-Host "Press Enter to exit"
        exit 1
    }
    
    Write-ColorOutput "" "White"
    
    # Execute based on parameters
    if ($DownloadOnly) {
        Write-ColorOutput "📥 Running codebase download only" "Cyan"
        Write-ColorOutput "" "White"
        
        $result = Invoke-PythonScript "tests/download_real_codebases.py"
        if (-not $result.Success) {
            Write-ColorOutput "❌ Download failed. Check the output above for details." "Red"
            Read-Host "Press Enter to exit"
            exit 1
        }
        
    } elseif ($Phase) {
        Write-ColorOutput "🎯 Running specific phase: $Phase" "Cyan"
        Write-ColorOutput "" "White"
        
        $result = Invoke-PythonScript "tests/run_comprehensive_compression_tests.py" @("--phase", $Phase)
        if (-not $result.Success) {
            Write-ColorOutput "❌ Phase execution failed. Check the output above for details." "Red"
            Read-Host "Press Enter to exit"
            exit 1
        }
        
    } else {
        Write-ColorOutput "🚀 Running comprehensive compression tests across all phases" "Cyan"
        Write-ColorOutput "" "White"
        Write-ColorOutput "This will:" "Yellow"
        Write-ColorOutput "  1. Download real codebases from 10 programming languages" "White"
        Write-ColorOutput "  2. Run multi-language compression tests" "White"
        Write-ColorOutput "  3. Run real compression integration tests" "White"
        Write-ColorOutput "" "White"
        Write-ColorOutput "First run may take 10-30 minutes to download codebases" "Yellow"
        Write-ColorOutput "" "White"
        
        $args = @()
        if ($SkipDownload) {
            $args += "--skip-download"
        }
        
        $result = Invoke-PythonScript "tests/run_comprehensive_compression_tests.py" $args
        if (-not $result.Success) {
            Write-ColorOutput "❌ Comprehensive testing failed. Check the output above for details." "Red"
            Read-Host "Press Enter to exit"
            exit 1
        }
    }
    
    Write-ColorOutput "" "White"
    Write-ColorOutput "================================================================================" "Cyan"
    Write-ColorOutput "Test execution completed" "Cyan"
    Write-ColorOutput "================================================================================" "Cyan"
    Write-ColorOutput "" "White"
    Write-ColorOutput "📄 Reports generated:" "Yellow"
    Write-ColorOutput "  - comprehensive_test_report.md" "White"
    Write-ColorOutput "  - comprehensive_test_results.json" "White"
    Write-ColorOutput "  - test_report.md (multi-language)" "White"
    Write-ColorOutput "  - real_compression_test_report.md (real integration)" "White"
    Write-ColorOutput "  - codebase_download_report.md (download)" "White"
    Write-ColorOutput "" "White"
    
    Read-Host "Press Enter to exit"
}

# Run main function
try {
    Main
} catch {
    Write-ColorOutput "❌ Unexpected error occurred: $($_.Exception.Message)" "Red"
    Write-ColorOutput "Stack trace: $($_.ScriptStackTrace)" "Red"
    Read-Host "Press Enter to exit"
    exit 1
}
