#!/usr/bin/env python3
"""
Download Real Codebases for Multi-Language Testing
Downloads real, production codebases from multiple programming languages for comprehensive compression testing
"""

import os
import sys
import subprocess
import json
import time
from pathlib import Path
from typing import Dict, List, Tuple, Any
import tempfile
import shutil

class RealCodebaseDownloader:
    """Downloads real codebases from multiple programming languages for testing"""
    
    def __init__(self):
        self.download_dir = Path("test_codebases")
        self.download_dir.mkdir(exist_ok=True)
        
        # Real codebases to download for comprehensive testing
        self.codebases = {
            'python': {
                'name': 'Python',
                'repos': [
                    {'name': 'cpython', 'url': 'https://github.com/python/cpython.git', 'branch': 'main'},
                    {'name': 'django', 'url': 'https://github.com/django/django.git', 'branch': 'main'},
                    {'name': 'flask', 'url': 'https://github.com/pallets/flask.git', 'branch': 'main'},
                    {'name': 'requests', 'url': 'https://github.com/psf/requests.git', 'branch': 'main'}
                ],
                'file_extensions': ['.py', '.pyx', '.pxd'],
                'expected_patterns': 40
            },
            'javascript': {
                'name': 'JavaScript',
                'repos': [
                    {'name': 'node', 'url': 'https://github.com/nodejs/node.git', 'branch': 'main'},
                    {'name': 'react', 'url': 'https://github.com/facebook/react.git', 'branch': 'main'},
                    {'name': 'vue', 'url': 'https://github.com/vuejs/vue.git', 'branch': 'main'},
                    {'name': 'lodash', 'url': 'https://github.com/lodash/lodash.git', 'branch': 'main'}
                ],
                'file_extensions': ['.js', '.ts', '.jsx', '.tsx'],
                'expected_patterns': 35
            },
            'java': {
                'name': 'Java',
                'repos': [
                    {'name': 'spring-boot', 'url': 'https://github.com/spring-projects/spring-boot.git', 'branch': 'main'},
                    {'name': 'hibernate', 'url': 'https://github.com/hibernate/hibernate-orm.git', 'branch': 'main'},
                    {'name': 'guava', 'url': 'https://github.com/google/guava.git', 'branch': 'main'},
                    {'name': 'junit5', 'url': 'https://github.com/junit-team/junit5.git', 'branch': 'main'}
                ],
                'file_extensions': ['.java'],
                'expected_patterns': 45
            },
            'cpp': {
                'name': 'C++',
                'repos': [
                    {'name': 'llvm-project', 'url': 'https://github.com/llvm/llvm-project.git', 'branch': 'main'},
                    {'name': 'boost', 'url': 'https://github.com/boostorg/boost.git', 'branch': 'develop'},
                    {'name': 'qt', 'url': 'https://github.com/qt/qt5.git', 'branch': '5.15'},
                    {'name': 'opencv', 'url': 'https://github.com/opencv/opencv.git', 'branch': '4.x'}
                ],
                'file_extensions': ['.cpp', '.hpp', '.cc', '.hh', '.c', '.h'],
                'expected_patterns': 40
            },
            'go': {
                'name': 'Go',
                'repos': [
                    {'name': 'kubernetes', 'url': 'https://github.com/kubernetes/kubernetes.git', 'branch': 'main'},
                    {'name': 'docker', 'url': 'https://github.com/moby/moby.git', 'branch': 'main'},
                    {'name': 'etcd', 'url': 'https://github.com/etcd-io/etcd.git', 'branch': 'main'},
                    {'name': 'prometheus', 'url': 'https://github.com/prometheus/prometheus.git', 'branch': 'main'}
                ],
                'file_extensions': ['.go'],
                'expected_patterns': 25
            },
            'csharp': {
                'name': 'C#',
                'repos': [
                    {'name': 'dotnet-runtime', 'url': 'https://github.com/dotnet/runtime.git', 'branch': 'main'},
                    {'name': 'aspnetcore', 'url': 'https://github.com/dotnet/aspnetcore.git', 'branch': 'main'},
                    {'name': 'entity-framework', 'url': 'https://github.com/dotnet/efcore.git', 'branch': 'main'},
                    {'name': 'roslyn', 'url': 'https://github.com/dotnet/roslyn.git', 'branch': 'main'}
                ],
                'file_extensions': ['.cs'],
                'expected_patterns': 35
            },
            'php': {
                'name': 'PHP',
                'repos': [
                    {'name': 'php-src', 'url': 'https://github.com/php/php-src.git', 'branch': 'master'},
                    {'name': 'laravel', 'url': 'https://github.com/laravel/laravel.git', 'branch': 'master'},
                    {'name': 'symfony', 'url': 'https://github.com/symfony/symfony.git', 'branch': '6.4'},
                    {'name': 'composer', 'url': 'https://github.com/composer/composer.git', 'branch': 'main'}
                ],
                'file_extensions': ['.php'],
                'expected_patterns': 30
            },
            'swift': {
                'name': 'Swift',
                'repos': [
                    {'name': 'swift', 'url': 'https://github.com/apple/swift.git', 'branch': 'main'},
                    {'name': 'swift-package-manager', 'url': 'https://github.com/apple/swift-package-manager.git', 'branch': 'main'},
                    {'name': 'swift-nio', 'url': 'https://github.com/apple/swift-nio.git', 'branch': 'main'},
                    {'name': 'vapor', 'url': 'https://github.com/vapor/vapor.git', 'branch': 'main'}
                ],
                'file_extensions': ['.swift'],
                'expected_patterns': 25
            },
            'kotlin': {
                'name': 'Kotlin',
                'repos': [
                    {'name': 'kotlin', 'url': 'https://github.com/JetBrains/kotlin.git', 'branch': 'main'},
                    {'name': 'ktor', 'url': 'https://github.com/ktorio/ktor.git', 'branch': 'main'},
                    {'name': 'kotlinx-coroutines', 'url': 'https://github.com/Kotlin/kotlinx.coroutines.git', 'branch': 'main'},
                    {'name': 'kotlinx-serialization', 'url': 'https://github.com/Kotlin/kotlinx.serialization.git', 'branch': 'main'}
                ],
                'file_extensions': ['.kt', '.kts'],
                'expected_patterns': 30
            },
            'rust': {
                'name': 'Rust',
                'repos': [
                    {'name': 'rust', 'url': 'https://github.com/rust-lang/rust.git', 'branch': 'master'},
                    {'name': 'tokio', 'url': 'https://github.com/tokio-rs/tokio.git', 'branch': 'master'},
                    {'name': 'serde', 'url': 'https://github.com/serde-rs/serde.git', 'branch': 'master'},
                    {'name': 'clap', 'url': 'https://github.com/clap-rs/clap.git', 'branch': 'master'}
                ],
                'file_extensions': ['.rs', '.toml'],
                'expected_patterns': 50
            }
        }
        
        self.downloaded_repos = {}
        
    def log_action(self, action: str, details: str = ""):
        """Log actions with timestamps"""
        timestamp = time.strftime("%Y-%m-%dT%H:%M:%SZ", time.gmtime())
        log_entry = f"{timestamp} | {action} | {details}"
        print(f"LOG: {log_entry}")
        
        # Append to recent-sync.md
        with open("logs/recent-sync.md", "a", encoding="utf-8") as f:
            f.write(f"\n### **{log_entry}**\n")
    
    def check_git_available(self) -> bool:
        """Check if git is available on the system"""
        try:
            result = subprocess.run(['git', '--version'], capture_output=True, text=True)
            return result.returncode == 0
        except FileNotFoundError:
            return False
    
    def download_repository(self, repo_info: Dict, language: str) -> bool:
        """Download a single repository"""
        repo_name = repo_info['name']
        repo_url = repo_info['url']
        repo_branch = repo_info['branch']
        
        repo_dir = self.download_dir / language / repo_name
        
        try:
            if repo_dir.exists():
                self.log_action(f"REPO_EXISTS", f"{language}/{repo_name} already exists, updating...")
                # Update existing repository
                subprocess.run(['git', 'fetch', 'origin'], cwd=repo_dir, check=True)
                subprocess.run(['git', 'checkout', repo_branch], cwd=repo_dir, check=True)
                subprocess.run(['git', 'pull', 'origin', repo_branch], cwd=repo_dir, check=True)
            else:
                self.log_action(f"DOWNLOADING_REPO", f"Downloading {language}/{repo_name} from {repo_url}")
                # Clone new repository
                repo_dir.parent.mkdir(parents=True, exist_ok=True)
                subprocess.run([
                    'git', 'clone', '--branch', repo_branch, '--depth', '1', 
                    repo_url, str(repo_dir)
                ], check=True)
            
            # Get repository info
            commit_hash = subprocess.check_output(
                ['git', 'rev-parse', 'HEAD'], 
                cwd=repo_dir, 
                text=True
            ).strip()
            
            self.downloaded_repos[f"{language}/{repo_name}"] = {
                'path': str(repo_dir),
                'commit': commit_hash,
                'url': repo_url,
                'branch': repo_branch,
                'language': language
            }
            
            self.log_action(f"REPO_SUCCESS", f"{language}/{repo_name} downloaded successfully (commit: {commit_hash[:8]})")
            return True
            
        except subprocess.CalledProcessError as e:
            self.log_action(f"REPO_ERROR", f"Failed to download {language}/{repo_name}: {str(e)}")
            return False
        except Exception as e:
            self.log_action(f"REPO_ERROR", f"Unexpected error downloading {language}/{repo_name}: {str(e)}")
            return False
    
    def download_language_codebases(self, language: str, config: Dict) -> Dict[str, Any]:
        """Download all codebases for a specific language"""
        self.log_action(f"DOWNLOADING_LANGUAGE", f"Starting downloads for {language}")
        
        results = []
        successful_downloads = 0
        
        for repo_info in config['repos']:
            success = self.download_repository(repo_info, language)
            results.append({
                'repo_name': repo_info['name'],
                'repo_url': repo_info['url'],
                'success': success
            })
            
            if success:
                successful_downloads += 1
        
        language_results = {
            'language': language,
            'total_repos': len(config['repos']),
            'successful_downloads': successful_downloads,
            'success_rate': (successful_downloads / len(config['repos'])) * 100,
            'repo_results': results
        }
        
        self.log_action(f"LANGUAGE_DOWNLOAD_COMPLETE", 
                       f"{language}: {successful_downloads}/{len(config['repos'])} repos downloaded successfully")
        return language_results
    
    def download_all_codebases(self) -> Dict[str, Any]:
        """Download all codebases for all languages"""
        self.log_action("DOWNLOAD_ALL_START", "Beginning download of all codebases across 10 languages")
        
        if not self.check_git_available():
            raise Exception("Git is not available on the system. Please install git to download codebases.")
        
        overall_stats = {
            'total_languages': len(self.codebases),
            'languages_processed': 0,
            'total_repos': sum(len(config['repos']) for config in self.codebases.values()),
            'successful_downloads': 0,
            'overall_success_rate': 0,
            'language_results': {}
        }
        
        total_repos = 0
        total_successful = 0
        
        for language, config in self.codebases.items():
            try:
                language_result = self.download_language_codebases(language, config)
                overall_stats['language_results'][language] = language_result
                
                total_repos += language_result['total_repos']
                total_successful += language_result['successful_downloads']
                
                if language_result['successful_downloads'] > 0:
                    overall_stats['languages_processed'] += 1
                    
            except Exception as e:
                self.log_action(f"LANGUAGE_DOWNLOAD_ERROR", f"{language}: {str(e)}")
                overall_stats['language_results'][language] = {'success': False, 'error': str(e)}
        
        # Calculate overall statistics
        if total_repos > 0:
            overall_stats['overall_success_rate'] = (total_successful / total_repos) * 100
        
        overall_stats['total_repos'] = total_repos
        overall_stats['successful_downloads'] = total_successful
        
        return overall_stats
    
    def generate_download_report(self, stats: Dict[str, Any]) -> str:
        """Generate a comprehensive download report"""
        report = []
        report.append("# REAL CODEBASE DOWNLOAD REPORT")
        report.append(f"**Download Date:** {time.strftime('%Y-%m-%d %H:%M:%S UTC', time.gmtime())}")
        report.append(f"**Total Languages:** {stats['total_languages']}")
        report.append(f"**Total Repositories:** {stats['total_repos']}")
        report.append(f"**Successfully Downloaded:** {stats['successful_downloads']}")
        report.append(f"**Overall Success Rate:** {stats['overall_success_rate']:.1f}%")
        report.append("")
        
        # Language-by-Language Results
        report.append("## 🌍 LANGUAGE-BY-LANGUAGE DOWNLOAD RESULTS")
        for language, result in stats['language_results'].items():
            if 'error' not in result:
                report.append(f"### {result['language'].upper()}")
                report.append(f"- **Repositories:** {result['total_repos']}")
                report.append(f"- **Successful Downloads:** {result['successful_downloads']}")
                report.append(f"- **Success Rate:** {result['success_rate']:.1f}%")
                report.append("")
            else:
                report.append(f"### {language.upper()} - ❌ FAILED")
                report.append(f"- **Error:** {result['error']}")
                report.append("")
        
        # Repository Details
        report.append("## 📁 DOWNLOADED REPOSITORIES")
        for repo_path, repo_info in self.downloaded_repos.items():
            report.append(f"### {repo_path}")
            report.append(f"- **URL:** {repo_info['url']}")
            report.append(f"- **Branch:** {repo_info['branch']}")
            report.append(f"- **Commit:** {repo_info['commit'][:8]}")
            report.append(f"- **Path:** {repo_info['path']}")
            report.append("")
        
        # File Statistics
        report.append("## FILE STATISTICS")
        total_files = 0
        total_size = 0
        
        for repo_path, repo_info in self.downloaded_repos.items():
            repo_dir = Path(repo_info['path'])
            if repo_dir.exists():
                # Count files by extension
                for ext in self.codebases[repo_info['language']]['file_extensions']:
                    files = list(repo_dir.rglob(f"*{ext}"))
                    total_files += len(files)
                    
                    for file_path in files:
                        if file_path.is_file():
                            total_size += file_path.stat().st_size
        
        report.append(f"**Total Source Files:** {total_files:,}")
        report.append(f"**Total Size:** {total_size / (1024*1024):.1f} MB")
        report.append("")
        
        # Testing Instructions
        report.append("## READY FOR TESTING")
        report.append("The downloaded codebases are now ready for comprehensive compression testing.")
        report.append("")
        report.append("**Next Steps:**")
        report.append("1. Run the multi-language compression test suite")
        report.append("2. Test compression across all 10 programming languages")
        report.append("3. Verify structural integrity with real production code")
        report.append("4. Measure compression ratios on diverse codebases")
        
        return "\n".join(report)
    
    def save_download_info(self, filename: str = "downloaded_codebases.json"):
        """Save download information to JSON file"""
        with open(filename, 'w', encoding='utf-8') as f:
            json.dump(self.downloaded_repos, f, indent=2, default=str)
        
        self.log_action("DOWNLOAD_INFO_SAVED", f"Download information saved to {filename}")
    
    def run_download_and_report(self) -> str:
        """Run downloads and generate comprehensive report"""
        self.log_action("DOWNLOAD_EXECUTION_START", "Beginning download of real codebases for testing")
        
        # Download all codebases
        download_stats = self.download_all_codebases()
        
        # Generate report
        report = self.generate_download_report(download_stats)
        
        # Save download information
        self.save_download_info()
        
        # Log completion
        self.log_action("DOWNLOAD_EXECUTION_COMPLETE", 
                       f"Download complete: {download_stats['languages_processed']} languages, "
                       f"{download_stats['successful_downloads']}/{download_stats['total_repos']} repos")
        
        return report

def main():
    """Main execution function"""
    print("REAL CODEBASE DOWNLOADER")
    print("Downloading real production codebases from 10 programming languages for comprehensive testing")
    print("=" * 80)
    
    downloader = RealCodebaseDownloader()
    
    try:
        report = downloader.run_download_and_report()
        print("\n" + "=" * 80)
        print(report)
        print("\n" + "=" * 80)
        
        # Save report to file
        with open("codebase_download_report.md", "w", encoding="utf-8") as f:
            f.write(report)
        
        print("Download report saved to codebase_download_report.md")
        print("Download information saved to downloaded_codebases.json")
        print("Ready to run comprehensive compression tests across all languages!")
        
    except Exception as e:
        print(f"Download execution failed: {str(e)}")
        import traceback
        traceback.print_exc()
        return 1
    
    return 0

if __name__ == "__main__":
    sys.exit(main())
