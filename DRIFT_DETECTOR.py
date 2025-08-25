#!/usr/bin/env python3
"""
DRIFT DETECTOR - NEXUS Compression System
Automatically detects drift and prevents agents from working on wrong files

Usage: python DRIFT_DETECTOR.py
"""

import os
import sys
from pathlib import Path
import re

class DriftDetector:
    def __init__(self):
        self.project_root = Path.cwd()
        self.drift_detected = False
        self.drift_report = []
        
        # Legitimate paths where code should exist
        self.legitimate_paths = {
            'core_system': ['src'],
            'enhanced_components': ['working_code'],
            'tests': ['tests'],
            'documentation': ['docs'],
            'logs': ['logs']
        }
        
        # Forbidden paths where code should NOT exist
        self.forbidden_paths = ['cleanup', 'temp', 'tmp', 'old', 'backup']
        
        # Core system files that should exist in specific locations
        self.core_files = {
            'nexus_compression_engine.rs': 'src',
            'enhanced_compression.rs': 'working_code',
            'ai_scheduler.rs': 'working_code',
            'neuromem.rs': 'working_code',
            'gpu_acceleration.rs': 'working_code'
        }
        
        # Test file patterns that should only be in /tests/
        self.test_patterns = [
            r'test_.*\.rs$',
            r'.*_test\.rs$',
            r'.*Test.*\.rs$',
            r'.*test.*\.py$',
            r'.*Test.*\.py$'
        ]

    def detect_drift(self):
        """Main drift detection function"""
        print("🔍 DRIFT DETECTOR - NEXUS Compression System")
        print("=" * 60)
        
        self.check_forbidden_paths()
        self.check_core_file_locations()
        self.check_test_file_locations()
        self.check_duplicate_implementations()
        self.check_alt_paths()
        
        return self.drift_detected

    def check_forbidden_paths(self):
        """Check if code exists in forbidden paths"""
        print("\n🔒 Checking forbidden paths...")
        
        for forbidden_path in self.forbidden_paths:
            path = self.project_root / forbidden_path
            if path.exists():
                # Look for code files in forbidden paths
                code_files = []
                for ext in ['.rs', '.py', '.js', '.ts', '.cpp', '.c', '.h']:
                    code_files.extend(path.rglob(f"*{ext}"))
                
                if code_files:
                    self.drift_detected = True
                    self.drift_report.append(f"🚨 CRITICAL: Code files found in forbidden path '{forbidden_path}':")
                    for file in code_files[:5]:  # Show first 5
                        self.drift_report.append(f"   - {file}")
                    if len(code_files) > 5:
                        self.drift_report.append(f"   ... and {len(code_files) - 5} more files")
                    print(f"   ❌ Found {len(code_files)} code files in '{forbidden_path}'")
                else:
                    print(f"   ✅ No code files in '{forbidden_path}'")

    def check_core_file_locations(self):
        """Check if core files are in their correct locations"""
        print("\n🎯 Checking core file locations...")
        
        for filename, expected_path in self.core_files.items():
            expected_file = self.project_root / expected_path / filename
            if not expected_file.exists():
                # Look for the file in other locations
                found_locations = []
                for path in self.project_root.rglob(filename):
                    found_locations.append(str(path.parent))
                
                if found_locations:
                    self.drift_detected = True
                    self.drift_report.append(f"🚨 DRIFT: Core file '{filename}' found in wrong location(s):")
                    for location in found_locations:
                        self.drift_report.append(f"   - {location}")
                    print(f"   ❌ '{filename}' found in wrong location(s): {found_locations}")
                else:
                    print(f"   ❌ '{filename}' not found anywhere")
            else:
                print(f"   ✅ '{filename}' in correct location: {expected_path}")

    def check_test_file_locations(self):
        """Check if test files are only in /tests/ folder"""
        print("\n🧪 Checking test file locations...")
        
        test_files_outside_tests = []
        
        for pattern in self.test_patterns:
            regex = re.compile(pattern, re.IGNORECASE)
            
            # Look for test files outside /tests/ folder
            for path in self.project_root.rglob("*"):
                if path.is_file() and regex.match(path.name):
                    # Check if it's outside /tests/ folder
                    if 'tests' not in str(path):
                        test_files_outside_tests.append(str(path))
        
        if test_files_outside_tests:
            self.drift_detected = True
            self.drift_report.append("🚨 DRIFT: Test files found outside /tests/ folder:")
            for file in test_files_outside_tests[:10]:  # Show first 10
                self.drift_report.append(f"   - {file}")
            if len(test_files_outside_tests) > 10:
                self.drift_report.append(f"   ... and {len(test_files_outside_tests) - 10} more files")
            print(f"   ❌ Found {len(test_files_outside_tests)} test files outside /tests/ folder")
        else:
            print("   ✅ All test files in correct locations")

    def check_duplicate_implementations(self):
        """Check for duplicate implementations of the same functionality"""
        print("\n🔄 Checking for duplicate implementations...")
        
        # Look for files with similar names that might be duplicates
        duplicate_candidates = {}
        
        for path in self.project_root.rglob("*.rs"):
            if path.is_file():
                # Extract base name without extension
                base_name = path.stem.lower()
                if base_name not in duplicate_candidates:
                    duplicate_candidates[base_name] = []
                duplicate_candidates[base_name].append(str(path))
        
        duplicates_found = False
        for base_name, files in duplicate_candidates.items():
            if len(files) > 1:
                # Check if these are actually duplicates (not just similar names)
                if any(keyword in base_name for keyword in ['compression', 'scheduler', 'neuromem', 'gpu']):
                    duplicates_found = True
                    self.drift_detected = True
                    self.drift_report.append(f"🚨 DRIFT: Potential duplicate implementation '{base_name}':")
                    for file in files:
                        self.drift_report.append(f"   - {file}")
                    print(f"   ❌ Potential duplicate: '{base_name}' found in {len(files)} locations")
        
        if not duplicates_found:
            print("   ✅ No duplicate implementations detected")

    def check_alt_paths(self):
        """Check for alternative path structures that could cause confusion"""
        print("\n🛤️ Checking for alternative paths...")
        
        alt_paths = []
        
        # Look for common alternative path patterns
        alt_patterns = [
            'alternative', 'alt', 'backup', 'old', 'new', 'enhanced', 'improved',
            'version', 'v2', 'v3', 'beta', 'alpha', 'experimental'
        ]
        
        for path in self.project_root.rglob("*"):
            if path.is_dir():
                path_name = path.name.lower()
                if any(pattern in path_name for pattern in alt_patterns):
                    # Check if this directory contains code files
                    code_files = list(path.rglob("*.rs")) + list(path.rglob("*.py"))
                    if code_files:
                        alt_paths.append((str(path), len(code_files)))
        
        if alt_paths:
            self.drift_detected = True
            self.drift_report.append("🚨 DRIFT: Alternative paths found that could cause confusion:")
            for path, count in alt_paths:
                self.drift_report.append(f"   - {path} ({count} code files)")
            print(f"   ❌ Found {len(alt_paths)} alternative paths with code files")
        else:
            print("   ✅ No alternative paths detected")

    def generate_report(self):
        """Generate comprehensive drift report"""
        if not self.drift_detected:
            print("\n🎉 NO DRIFT DETECTED - SYSTEM IS CLEAN!")
            print("✅ You can proceed with development")
            return
        
        print("\n🚨 DRIFT DETECTED - IMMEDIATE ACTION REQUIRED!")
        print("=" * 60)
        print("STOP ALL WORK IMMEDIATELY!")
        print("DO NOT ATTEMPT TO FIX DRIFT YOURSELF!")
        print("REPORT TO LOGS AND REQUEST CLEANUP!")
        print("=" * 60)
        
        print("\n📋 DRIFT REPORT:")
        for line in self.drift_report:
            print(line)
        
        print("\n🚫 IMMEDIATE ACTIONS REQUIRED:")
        print("1. STOP ALL DEVELOPMENT WORK")
        print("2. DOCUMENT DRIFT IN /logs/recent-sync.md")
        print("3. REQUEST CLEANUP FROM SYSTEM ADMINISTRATOR")
        print("4. WAIT FOR DRIFT TO BE RESOLVED")
        print("5. DO NOT ATTEMPT TO FIX DRIFT YOURSELF")
        
        print("\n🔒 DRIFT LOCKDOWN ACTIVE - ZERO TOLERANCE")

def main():
    """Main function"""
    detector = DriftDetector()
    
    try:
        drift_detected = detector.detect_drift()
        detector.generate_report()
        
        # Exit with error code if drift detected
        if drift_detected:
            sys.exit(1)
        else:
            sys.exit(0)
            
    except Exception as e:
        print(f"❌ Error during drift detection: {e}")
        print("🚨 Treating as potential drift - STOP ALL WORK")
        sys.exit(1)

if __name__ == "__main__":
    main()
