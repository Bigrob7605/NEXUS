#!/usr/bin/env python3
"""
Model Setup Script for The 12: Multi-Layered Consensus System
Helps configure and validate the 12 quantized models
"""

import os
import sys
import json
import logging
from pathlib import Path
from typing import Dict, List, Optional
import subprocess
import shutil

# Configure logging
logging.basicConfig(level=logging.INFO, format='%(asctime)s - %(levelname)s - %(message)s')
logger = logging.getLogger(__name__)

class ModelSetup:
    """Handles model setup and validation"""
    
    def __init__(self):
        self.models_dir = Path("models")
        self.models_dir.mkdir(exist_ok=True)
        
        # Model configurations with download sources and validation
        self.model_configs = {
            'llama-3.1-8b': {
                'type': 'generalist',
                'size_gb': 4.5,
                'quantization': 'Q4_K_M',
                'huggingface_id': 'TheBloke/Llama-3.1-8B-GGUF',
                'filename': 'llama-3.1-8b.Q4_K_M.gguf',
                'description': 'Meta\'s latest generalist model, excellent reasoning'
            },
            'mistral-7b': {
                'type': 'generalist',
                'size_gb': 4.0,
                'quantization': 'Q4_K_M',
                'huggingface_id': 'TheBloke/Mistral-7B-Instruct-v0.2-GGUF',
                'filename': 'mistral-7b-instruct-v0.2.Q4_K_M.gguf',
                'description': 'Mistral AI\'s efficient 7B model'
            },
            'gemma-2-9b': {
                'type': 'generalist',
                'size_gb': 5.5,
                'quantization': 'Q4_K_M',
                'huggingface_id': 'TheBloke/gemma-2-9B-GGUF',
                'filename': 'gemma-2-9b.Q4_K_M.gguf',
                'description': 'Google\'s Gemma 2, excellent for general tasks'
            },
            'deepseek-coder-7b': {
                'type': 'reasoning',
                'size_gb': 4.0,
                'quantization': 'Q4_K_M',
                'huggingface_id': 'TheBloke/deepseek-coder-7B-instruct-GGUF',
                'filename': 'deepseek-coder-7b-instruct.Q4_K_M.gguf',
                'description': 'Specialized for coding and reasoning tasks'
            },
            'phi-3-14b': {
                'type': 'reasoning',
                'size_gb': 7.5,
                'quantization': 'Q4_K_M',
                'huggingface_id': 'TheBloke/Phi-3-14B-GGUF',
                'filename': 'phi-3-14b.Q4_K_M.gguf',
                'description': 'Microsoft\'s Phi-3, excellent reasoning capabilities'
            },
            'wizardmath-7b': {
                'type': 'stem',
                'size_gb': 4.0,
                'quantization': 'Q4_K_M',
                'huggingface_id': 'TheBloke/WizardMath-7B-V1.0-GGUF',
                'filename': 'wizardmath-7b-v1.0.Q4_K_M.gguf',
                'description': 'Specialized for mathematical reasoning'
            },
            'codellama-13b': {
                'type': 'stem',
                'size_gb': 7.0,
                'quantization': 'Q4_K_M',
                'huggingface_id': 'TheBloke/CodeLlama-13B-Python-GGUF',
                'filename': 'codellama-13b-python.Q4_K_M.gguf',
                'description': 'Meta\'s CodeLlama, excellent for programming'
            },
            'openhermes-7b': {
                'type': 'humanities',
                'size_gb': 4.0,
                'quantization': 'Q4_K_M',
                'huggingface_id': 'TheBloke/OpenHermes-7B-GGUF',
                'filename': 'openhermes-7b.Q4_K_M.gguf',
                'description': 'Specialized for humanities and creative tasks'
            },
            'dolphin-2.9-8b': {
                'type': 'humanities',
                'size_gb': 4.5,
                'quantization': 'Q4_K_M',
                'huggingface_id': 'TheBloke/dolphin-2.9-8B-GGUF',
                'filename': 'dolphin-2.9-8b.Q4_K_M.gguf',
                'description': 'Excellent for creative writing and analysis'
            },
            'zephyr-7b': {
                'type': 'emotional',
                'size_gb': 4.0,
                'quantization': 'Q4_K_M',
                'huggingface_id': 'TheBloke/zephyr-7B-beta-GGUF',
                'filename': 'zephyr-7b-beta.Q4_K_M.gguf',
                'description': 'Hugging Face\'s Zephyr, good emotional intelligence'
            },
            'nous-hermes-7b': {
                'type': 'emotional',
                'size_gb': 4.0,
                'quantization': 'Q4_K_M',
                'huggingface_id': 'TheBloke/Nous-Hermes-7B-GGUF',
                'filename': 'nous-hermes-7b.Q4_K_M.gguf',
                'description': 'Nous Research model, balanced emotional reasoning'
            },
            'steerlm-7b': {
                'type': 'emotional',
                'size_gb': 4.0,
                'quantization': 'Q4_K_M',
                'huggingface_id': 'TheBloke/SteerLM-7B-GGUF',
                'filename': 'steerlm-7b.Q4_K_M.gguf',
                'description': 'Microsoft\'s SteerLM, excellent emotional steering'
            }
        }
    
    def check_disk_space(self) -> bool:
        """Check if there's enough disk space for all models"""
        try:
            # Get available space on models directory
            total, used, free = shutil.disk_usage(self.models_dir)
            free_gb = free / (1024**3)
            
            # Calculate total space needed
            total_needed = sum(config['size_gb'] for config in self.model_configs.values())
            
            logger.info(f"Available disk space: {free_gb:.1f} GB")
            logger.info(f"Total space needed: {total_needed:.1f} GB")
            
            if free_gb < total_needed:
                logger.warning(f"Insufficient disk space! Need {total_needed:.1f} GB, have {free_gb:.1f} GB")
                return False
            
            return True
            
        except Exception as e:
            logger.error(f"Failed to check disk space: {e}")
            return False
    
    def check_existing_models(self) -> Dict[str, bool]:
        """Check which models are already downloaded"""
        existing_models = {}
        
        for model_name, config in self.model_configs.items():
            model_path = self.models_dir / config['filename']
            existing_models[model_name] = model_path.exists()
            
            if existing_models[model_name]:
                size_mb = model_path.stat().st_size / (1024**2)
                logger.info(f"✓ {model_name}: {size_mb:.1f} MB")
            else:
                logger.info(f"✗ {model_name}: Not found")
        
        return existing_models
    
    def install_huggingface_hub(self) -> bool:
        """Install huggingface_hub if not available"""
        try:
            import huggingface_hub
            logger.info("✓ huggingface_hub already installed")
            return True
        except ImportError:
            logger.info("Installing huggingface_hub...")
            try:
                subprocess.check_call([sys.executable, "-m", "pip", "install", "huggingface_hub"])
                logger.info("✓ huggingface_hub installed successfully")
                return True
            except subprocess.CalledProcessError as e:
                logger.error(f"Failed to install huggingface_hub: {e}")
                return False
    
    def download_model(self, model_name: str) -> bool:
        """Download a specific model from Hugging Face"""
        if model_name not in self.model_configs:
            logger.error(f"Unknown model: {model_name}")
            return False
        
        config = self.model_configs[model_name]
        model_path = self.models_dir / config['filename']
        
        if model_path.exists():
            logger.info(f"Model {model_name} already exists, skipping download")
            return True
        
        try:
            logger.info(f"Downloading {model_name}...")
            
            # Use huggingface_hub to download
            from huggingface_hub import hf_hub_download
            
            hf_hub_download(
                repo_id=config['huggingface_id'],
                filename=config['filename'],
                local_dir=self.models_dir,
                local_dir_use_symlinks=False
            )
            
            if model_path.exists():
                size_mb = model_path.stat().st_size / (1024**2)
                logger.info(f"✓ Downloaded {model_name}: {size_mb:.1f} MB")
                return True
            else:
                logger.error(f"Download failed for {model_name}")
                return False
                
        except Exception as e:
            logger.error(f"Failed to download {model_name}: {e}")
            return False
    
    def download_all_models(self) -> Dict[str, bool]:
        """Download all missing models"""
        logger.info("Starting download of all missing models...")
        
        # Check existing models first
        existing = self.check_existing_models()
        missing = [name for name, exists in existing.items() if not exists]
        
        if not missing:
            logger.info("All models are already downloaded!")
            return existing
        
        logger.info(f"Need to download {len(missing)} models: {', '.join(missing)}")
        
        # Download missing models
        results = {}
        for model_name in missing:
            results[model_name] = self.download_model(model_name)
        
        return results
    
    def validate_models(self) -> Dict[str, bool]:
        """Validate that all models are properly downloaded and accessible"""
        logger.info("Validating downloaded models...")
        
        validation_results = {}
        
        for model_name, config in self.model_configs.items():
            model_path = self.models_dir / config['filename']
            
            if not model_path.exists():
                validation_results[model_name] = False
                logger.error(f"✗ {model_name}: File not found")
                continue
            
            try:
                # Check file size (should be reasonable)
                size_mb = model_path.stat().st_size / (1024**2)
                expected_size_gb = config['size_gb']
                expected_size_mb = expected_size_gb * 1024
                
                # Allow 20% tolerance
                tolerance = expected_size_mb * 0.2
                min_size = expected_size_mb - tolerance
                max_size = expected_size_mb + tolerance
                
                if min_size <= size_mb <= max_size:
                    validation_results[model_name] = True
                    logger.info(f"✓ {model_name}: {size_mb:.1f} MB (expected ~{expected_size_gb:.1f} GB)")
                else:
                    validation_results[model_name] = False
                    logger.warning(f"⚠ {model_name}: Size {size_mb:.1f} MB outside expected range ({min_size:.1f}-{max_size:.1f} MB)")
                    
            except Exception as e:
                validation_results[model_name] = False
                logger.error(f"✗ {model_name}: Validation failed - {e}")
        
        return validation_results
    
    def create_model_registry(self) -> bool:
        """Create a JSON registry of all models"""
        try:
            registry = {}
            
            for model_name, config in self.model_configs.items():
                model_path = self.models_dir / config['filename']
                
                registry[model_name] = {
                    'type': config['type'],
                    'quantization': config['quantization'],
                    'filename': config['filename'],
                    'path': str(model_path),
                    'exists': model_path.exists(),
                    'size_mb': model_path.stat().st_size / (1024**2) if model_path.exists() else 0,
                    'description': config['description']
                }
            
            registry_path = self.models_dir / "model_registry.json"
            with open(registry_path, 'w') as f:
                json.dump(registry, f, indent=2)
            
            logger.info(f"✓ Model registry created: {registry_path}")
            return True
            
        except Exception as e:
            logger.error(f"Failed to create model registry: {e}")
            return False
    
    def run_setup(self) -> bool:
        """Run the complete setup process"""
        logger.info("=== The 12: Model Setup ===")
        
        # Check disk space
        if not self.check_disk_space():
            return False
        
        # Install dependencies
        if not self.install_huggingface_hub():
            return False
        
        # Check existing models
        logger.info("\n--- Checking Existing Models ---")
        existing = self.check_existing_models()
        
        # Download missing models
        logger.info("\n--- Downloading Missing Models ---")
        download_results = self.download_all_models()
        
        # Validate all models
        logger.info("\n--- Validating Models ---")
        validation_results = self.validate_models()
        
        # Create registry
        logger.info("\n--- Creating Model Registry ---")
        self.create_model_registry()
        
        # Summary
        logger.info("\n=== Setup Summary ===")
        total_models = len(self.model_configs)
        downloaded = sum(1 for exists in existing.values() if exists)
        valid = sum(1 for valid in validation_results.values() if valid)
        
        logger.info(f"Total models: {total_models}")
        logger.info(f"Downloaded: {downloaded}")
        logger.info(f"Valid: {valid}")
        
        if valid == total_models:
            logger.info("🎉 All models are ready!")
            return True
        else:
            logger.warning(f"⚠ {total_models - valid} models have issues")
            return False

def main():
    """Main entry point"""
    setup = ModelSetup()
    
    try:
        success = setup.run_setup()
        if success:
            logger.info("Model setup completed successfully!")
            sys.exit(0)
        else:
            logger.error("Model setup failed!")
            sys.exit(1)
    except KeyboardInterrupt:
        logger.info("Setup interrupted by user")
        sys.exit(1)
    except Exception as e:
        logger.error(f"Setup failed with error: {e}")
        sys.exit(1)

if __name__ == "__main__":
    main()
