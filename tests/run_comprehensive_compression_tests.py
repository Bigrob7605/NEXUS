#!/usr/bin/env python3
"""
Comprehensive Compression Test Runner
Orchestrates the entire multi-language compression testing process with real codebases
"""

import os
import sys
import subprocess
import json
import time
from pathlib import Path
from typing import Dict, List, Tuple, Any
import argparse

class ComprehensiveCompressionTestRunner:
    """Orchestrates comprehensive compression testing across multiple languages"""
    
    def __init__(self):
        self.test_start_time = time.time()
        self.overall_results = {}
        self.test_phases = []
        
        # Test phases in order
        self.phases = [
            {
                'name': 'download_codebases',
                'description': 'Download real codebases from 10 programming languages',
                'script': 'tests/download_real_codebases.py',
                'required': True
            },
            {
                'name': 'multi_language_test',
                'description': 'Run multi-language compression tests with simulation',
                'script': 'tests/multi_language_compression_test.py',
                'required': True
            },
            {
                'name': 'real_integration_test',
                'description': 'Run real compression integration tests',
                'script': 'tests/real_compression_integration_test.py',
                'required': True
            }
        ]
    
    def log_action(self, action: str, details: str = ""):
        """Log actions with timestamps"""
        timestamp = time.strftime("%Y-%m-%dT%H:%M:%SZ", time.gmtime())
        log_entry = f"{timestamp} | {action} | {details}"
        print(f"LOG: {log_entry}")
        
        # Append to recent-sync.md
        with open("logs/recent-sync.md", "a", encoding="utf-8") as f:
            f.write(f"\n### **{log_entry}**\n")
    
    def check_prerequisites(self) -> bool:
        """Check if all prerequisites are met"""
        self.log_action("PREREQUISITE_CHECK", "Checking system prerequisites")
        
        # Check Python version
        if sys.version_info < (3, 7):
            self.log_action("PREREQUISITE_ERROR", "Python 3.7+ required")
            return False
        
        # Check if required directories exist
        required_dirs = ['src', 'tests', 'logs']
        for dir_name in required_dirs:
            if not os.path.exists(dir_name):
                self.log_action("PREREQUISITE_ERROR", f"Required directory '{dir_name}' not found")
                return False
        
        # Check if Rust is available
        try:
            result = subprocess.run(['cargo', '--version'], capture_output=True, text=True)
            if result.returncode != 0:
                self.log_action("PREREQUISITE_WARNING", "Rust/Cargo not available - some tests may fail")
        except FileNotFoundError:
            self.log_action("PREREQUISITE_WARNING", "Rust/Cargo not available - some tests may fail")
        
        # Check if git is available
        try:
            result = subprocess.run(['git', '--version'], capture_output=True, text=True)
            if result.returncode != 0:
                self.log_action("PREREQUISITE_ERROR", "Git not available - cannot download codebases")
                return False
        except FileNotFoundError:
            self.log_action("PREREQUISITE_ERROR", "Git not available - cannot download codebases")
            return False
        
        self.log_action("PREREQUISITE_SUCCESS", "All prerequisites met")
        return True
    
    def run_test_phase(self, phase: Dict) -> Dict[str, Any]:
        """Run a single test phase"""
        phase_name = phase['name']
        script_path = phase['script']
        
        self.log_action(f"PHASE_START", f"Starting phase: {phase_name} - {phase['description']}")
        
        if not os.path.exists(script_path):
            return {
                'success': False,
                'error': f'Script not found: {script_path}',
                'phase': phase_name
            }
        
        try:
            # Run the test script
            start_time = time.time()
            result = subprocess.run([sys.executable, script_path], 
                                  capture_output=True, text=True, 
                                  cwd=os.getcwd())
            
            execution_time = time.time() - start_time
            
            if result.returncode == 0:
                self.log_action(f"PHASE_SUCCESS", f"{phase_name} completed successfully in {execution_time:.2f}s")
                return {
                    'success': True,
                    'phase': phase_name,
                    'execution_time': execution_time,
                    'stdout': result.stdout,
                    'stderr': result.stderr
                }
            else:
                self.log_action(f"PHASE_FAILED", f"{phase_name} failed with return code {result.returncode}")
                return {
                    'success': False,
                    'phase': phase_name,
                    'error': f'Script failed with return code {result.returncode}',
                    'stdout': result.stdout,
                    'stderr': result.stderr,
                    'execution_time': execution_time
                }
                
        except Exception as e:
            self.log_action(f"PHASE_ERROR", f"{phase_name} failed with exception: {str(e)}")
            return {
                'success': False,
                'phase': phase_name,
                'error': str(e)
            }
    
    def run_all_phases(self, skip_download: bool = False) -> Dict[str, Any]:
        """Run all test phases in sequence"""
        self.log_action("COMPREHENSIVE_TEST_START", "Beginning comprehensive compression testing across all phases")
        
        overall_results = {
            'test_start_time': time.strftime("%Y-%m-%dT%H:%M:%SZ", time.gmtime()),
            'total_phases': len(self.phases),
            'phases_completed': 0,
            'phases_failed': 0,
            'overall_success': True,
            'phase_results': {},
            'total_execution_time': 0
        }
        
        total_execution_time = 0
        
        for i, phase in enumerate(self.phases):
            # Skip download phase if requested
            if skip_download and phase['name'] == 'download_codebases':
                self.log_action("PHASE_SKIPPED", f"Skipping {phase['name']} as requested")
                overall_results['phase_results'][phase['name']] = {
                    'success': True,
                    'phase': phase['name'],
                    'skipped': True,
                    'reason': 'Skipped by user request'
                }
                continue
            
            # Run the phase
            phase_result = self.run_test_phase(phase)
            overall_results['phase_results'][phase['name']] = phase_result
            
            if phase_result['success']:
                overall_results['phases_completed'] += 1
                if 'execution_time' in phase_result:
                    total_execution_time += phase_result['execution_time']
            else:
                overall_results['phases_failed'] += 1
                if phase['required']:
                    overall_results['overall_success'] = False
                    self.log_action("CRITICAL_PHASE_FAILED", f"Required phase {phase['name']} failed - stopping execution")
                    break
        
        overall_results['total_execution_time'] = total_execution_time
        overall_results['test_end_time'] = time.strftime("%Y-%m-%dT%H:%M:%SZ", time.gmtime())
        
        self.overall_results = overall_results
        return overall_results
    
    def generate_comprehensive_report(self) -> str:
        """Generate a comprehensive test report"""
        if not self.overall_results:
            return "No test results available"
        
        report = []
        report.append("# COMPREHENSIVE COMPRESSION TEST REPORT")
        report.append(f"**Test Start:** {self.overall_results['test_start_time']}")
        report.append(f"**Test End:** {self.overall_results['test_end_time']}")
        report.append(f"**Total Duration:** {self.overall_results['total_execution_time']:.2f} seconds")
        report.append(f"**Phases Completed:** {self.overall_results['phases_completed']}/{self.overall_results['total_phases']}")
        report.append(f"**Phases Failed:** {self.overall_results['phases_failed']}")
        report.append(f"**Overall Success:** {'YES' if self.overall_results['overall_success'] else 'NO'}")
        report.append("")
        
        # Phase-by-Phase Results
        report.append("## PHASE-BY-PHASE RESULTS")
        for phase_name, phase_result in self.overall_results['phase_results'].items():
            if phase_result.get('skipped'):
                report.append(f"### {phase_name.upper()} - SKIPPED")
                report.append(f"- **Reason:** {phase_result['reason']}")
                report.append("")
            elif phase_result['success']:
                report.append(f"### {phase_name.upper()} - SUCCESS")
                if 'execution_time' in phase_result:
                    report.append(f"- **Execution Time:** {phase_result['execution_time']:.2f}s")
                if phase_result.get('stdout'):
                    report.append(f"- **Output:** {len(phase_result['stdout'])} characters")
                report.append("")
            else:
                report.append(f"### {phase_name.upper()} - FAILED")
                report.append(f"- **Error:** {phase_result['error']}")
                if phase_result.get('stderr'):
                    report.append(f"- **Error Output:** {phase_result['stderr']}")
                report.append("")
        
        # Overall Assessment
        report.append("## OVERALL ASSESSMENT")
        if self.overall_results['overall_success']:
            report.append("**ALL REQUIRED PHASES COMPLETED SUCCESSFULLY!**")
            report.append("")
            report.append("The comprehensive compression testing has been completed successfully.")
            report.append("All critical phases passed, and the system is ready for production use.")
        else:
            report.append("**SOME REQUIRED PHASES FAILED**")
            report.append("")
            report.append("The comprehensive testing encountered failures in required phases.")
            report.append("Please review the failed phases and address the issues before proceeding.")
        
        report.append("")
        
        # Next Steps
        report.append("## NEXT STEPS")
        if self.overall_results['overall_success']:
            report.append("1. **Review Test Reports** - Examine detailed results from each phase")
            report.append("2. **Analyze Performance** - Review compression ratios and integrity rates")
            report.append("3. **Production Deployment** - System ready for production use")
            report.append("4. **Continuous Monitoring** - Monitor performance in production environment")
        else:
            report.append("1. **Review Failures** - Examine failed phase results")
            report.append("2. **Fix Issues** - Address problems in failed phases")
            report.append("3. **Re-run Tests** - Execute failed phases again")
            report.append("4. **Verify Fixes** - Ensure all issues are resolved")
        
        return "\n".join(report)
    
    def save_results(self, filename: str = "comprehensive_test_results.json"):
        """Save comprehensive test results to JSON file"""
        with open(filename, 'w', encoding='utf-8') as f:
            json.dump(self.overall_results, f, indent=2, default=str)
        
        self.log_action("RESULTS_SAVED", f"Comprehensive test results saved to {filename}")
    
    def run_comprehensive_test(self, skip_download: bool = False) -> str:
        """Run the complete comprehensive test suite"""
        self.log_action("COMPREHENSIVE_EXECUTION_START", "Beginning comprehensive compression testing")
        
        # Check prerequisites
        if not self.check_prerequisites():
            raise Exception("Prerequisites not met - cannot continue with testing")
        
        # Run all phases
        test_results = self.run_all_phases(skip_download)
        
        # Generate report
        report = self.generate_comprehensive_report()
        
        # Save results
        self.save_results()
        
        # Log completion
        self.log_action("COMPREHENSIVE_EXECUTION_COMPLETE", 
                       f"Comprehensive testing complete: {test_results['phases_completed']}/{test_results['total_phases']} phases successful")
        
        return report

def main():
    """Main execution function"""
    parser = argparse.ArgumentParser(description='Run comprehensive compression tests across multiple languages')
    parser.add_argument('--skip-download', action='store_true', 
                       help='Skip downloading codebases (use existing ones)')
    parser.add_argument('--phase', choices=['download', 'multi_language', 'real_integration'],
                       help='Run only a specific phase')
    
    args = parser.parse_args()
    
    print("COMPREHENSIVE COMPRESSION TEST RUNNER")
    print("Testing the fixed compression system across 10 programming languages with real codebases")
    print("=" * 80)
    
    runner = ComprehensiveCompressionTestRunner()
    
    try:
        if args.phase:
            # Run specific phase
            phase_map = {
                'download': 'download_codebases',
                'multi_language': 'multi_language_test',
                'real_integration': 'real_integration_test'
            }
            
            target_phase = phase_map[args.phase]
            phase_config = next((p for p in runner.phases if p['name'] == target_phase), None)
            
            if phase_config:
                result = runner.run_test_phase(phase_config)
                if result['success']:
                    print(f"Phase '{args.phase}' completed successfully")
                else:
                    print(f"Phase '{args.phase}' failed: {result['error']}")
                    return 1
            else:
                print(f"Phase '{args.phase}' not found")
                return 1
        else:
            # Run comprehensive test
            report = runner.run_comprehensive_test(skip_download=args.skip_download)
            print("\n" + "=" * 80)
            print(report)
            print("\n" + "=" * 80)
            
            # Save report to file
            with open("comprehensive_test_report.md", "w", encoding="utf-8") as f:
                f.write(report)
            
            print("Comprehensive test report saved to comprehensive_test_report.md")
            print("Comprehensive test results saved to comprehensive_test_results.json")
        
    except Exception as e:
        print(f"Comprehensive test execution failed: {str(e)}")
        import traceback
        traceback.print_exc()
        return 1
    
    return 0

if __name__ == "__main__":
    sys.exit(main())
