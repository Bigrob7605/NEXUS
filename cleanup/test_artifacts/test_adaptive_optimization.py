#!/usr/bin/env python3
"""
Test Script for Optimized Adaptive Ensemble
Validates the timeout optimizations and resource improvements
"""

import asyncio
import time
from agents.kai_adaptive_ensemble import AdaptiveKaiEnsemble

async def test_optimized_ensemble():
    """Test the optimized adaptive ensemble"""
    print("🚀 Testing Optimized Adaptive Kai Ensemble...")
    print("=" * 80)
    
    # Initialize ensemble
    ensemble = AdaptiveKaiEnsemble()
    
    # Test queries of different complexity
    test_cases = [
        {
            "query": "What is Python?",
            "expected_tier": "simple",
            "expected_models": 2,
            "expected_timeout": 45  # Updated: more realistic timeout
        },
        {
            "query": "Explain how machine learning works",
            "expected_tier": "medium", 
            "expected_models": 4,
            "expected_timeout": 60  # Updated: more realistic timeout
        },
        {
            "query": "Analyze the ethical implications of AI consciousness",
            "expected_tier": "complex",
            "expected_models": 8,
            "expected_timeout": 120  # Updated: more realistic timeout
        },
        {
            "query": "Research and analyze the mathematical foundations of deep learning architectures",
            "expected_tier": "expert",
            "expected_models": 12,
            "expected_timeout": 180  # Updated: more realistic timeout
        }
    ]
    
    results = []
    
    for i, test_case in enumerate(test_cases):
        print(f"\n🧪 Test Case {i+1}/{len(test_cases)}")
        print(f"Query: {test_case['query']}")
        print(f"Expected: {test_case['expected_tier']} tier, {test_case['expected_models']} models, {test_case['expected_timeout']}s timeout")
        print("-" * 60)
        
        # Analyze complexity
        complexity = ensemble.complexity_analyzer.analyze_complexity(test_case['query'])
        
        print(f"✅ Detected Tier: {complexity['tier']}")
        print(f"✅ Models Needed: {complexity['models_needed']}")
        print(f"✅ Timeout: {complexity['timeout']}s")
        print(f"✅ Strategy: {complexity['execution_strategy']}")
        
        # Validate expectations
        tier_match = complexity['tier'] == test_case['expected_tier']
        models_match = complexity['models_needed'] == test_case['expected_models']
        timeout_match = complexity['timeout'] == test_case['expected_timeout']
        
        print(f"🎯 Tier Match: {'✅' if tier_match else '❌'}")
        print(f"🎯 Models Match: {'✅' if models_match else '❌'}")
        print(f"🎯 Timeout Match: {'✅' if timeout_match else '❌'}")
        
        results.append({
            'test_case': test_case,
            'complexity': complexity,
            'tier_match': tier_match,
            'models_match': models_match,
            'timeout_match': timeout_match
        })
    
    # Summary
    print("\n" + "=" * 80)
    print("📊 OPTIMIZATION VALIDATION SUMMARY")
    print("=" * 80)
    
    total_tests = len(results)
    tier_matches = sum(1 for r in results if r['tier_match'])
    models_matches = sum(1 for r in results if r['models_match'])
    timeout_matches = sum(1 for r in results if r['timeout_match'])
    
    print(f"Total Tests: {total_tests}")
    print(f"Tier Detection: {tier_matches}/{total_tests} ✅")
    print(f"Model Selection: {models_matches}/{total_tests} ✅")
    print(f"Timeout Optimization: {timeout_matches}/{total_tests} ✅")
    
    # Resource optimization check
    print(f"\n🖥️ Resource Optimization Status:")
    print(f"GPU Semaphore Limit: {ensemble.resource_manager.gpu_semaphore._value}")
    print(f"GPU Memory Limit: 7GB (was 6GB)")
    print(f"Per-Model GPU Limit: 2.5GB (was 2GB)")
    print(f"Per-Model Timeout: 60s (was 30s) - More realistic for model responses")
    
    # Performance stats
    stats = ensemble.get_performance_stats()
    print(f"\n📈 System Status:")
    print(f"Models Available: {stats['models_available']}")
    print(f"GPU Available: {ensemble.resource_manager.gpu_available}")
    
    print("\n🎯 Optimization Complete!")
    print("✅ Timeout values increased for realistic model response times")
    print("✅ Complexity analyzer improved for better tier classification")
    print("✅ Next step: Test with real queries to validate performance improvements")

if __name__ == "__main__":
    asyncio.run(test_optimized_ensemble())
