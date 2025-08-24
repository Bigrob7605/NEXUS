#!/usr/bin/env python3
"""
Kai Core V8+ MythGraph Ledger
Transparency and audit trail system
"""

import asyncio
import hashlib
import json
import time
from datetime import datetime
from typing import Dict, List, Optional, Any
from pathlib import Path

class MythGraphEntry:
    """
    Individual entry in the MythGraph ledger
    """
    
    def __init__(self, entry_type: str, data: Dict, timestamp: Optional[str] = None):
        self.entry_type = entry_type
        self.data = data
        self.timestamp = timestamp or datetime.utcnow().isoformat()
        self.hash = None
        self.signature = None
        self.previous_hash = None
    
    def calculate_hash(self, previous_hash: Optional[str] = None) -> str:
        """Calculate cryptographic hash of entry"""
        self.previous_hash = previous_hash
        
        # Create hashable data
        hash_data = {
            "entry_type": self.entry_type,
            "data": self.data,
            "timestamp": self.timestamp,
            "previous_hash": self.previous_hash
        }
        
        # Calculate SHA-256 hash
        hash_string = json.dumps(hash_data, sort_keys=True)
        self.hash = hashlib.sha256(hash_string.encode()).hexdigest()
        
        return self.hash
    
    def sign_entry(self, private_key: str) -> str:
        """Sign entry with private key"""
        if not self.hash:
            self.calculate_hash()
        
        # In a real implementation, use proper cryptographic signing
        # This is a simplified version for demonstration
        signature_data = f"{self.hash}:{private_key}"
        self.signature = hashlib.sha256(signature_data.encode()).hexdigest()
        
        return self.signature
    
    def verify_signature(self, public_key: str) -> bool:
        """Verify entry signature"""
        if not self.signature:
            return False
        
        # In a real implementation, use proper cryptographic verification
        # This is a simplified version for demonstration
        expected_signature = hashlib.sha256(f"{self.hash}:{public_key}".encode()).hexdigest()
        return self.signature == expected_signature
    
    def to_dict(self) -> Dict:
        """Convert entry to dictionary"""
        return {
            "entry_type": self.entry_type,
            "data": self.data,
            "timestamp": self.timestamp,
            "hash": self.hash,
            "signature": self.signature,
            "previous_hash": self.previous_hash
        }

class MythGraphLedger:
    """
    MythGraph ledger implementation
    """
    
    def __init__(self, public_key: str, private_key: str):
        self.entries: List[MythGraphEntry] = []
        self.public_key = public_key
        self.private_key = private_key
        self.ledger_hash = "0000000000000000000000000000000000000000000000000000000000000000"
        self.storage_path = Path("mythgraph")
        self.storage_path.mkdir(exist_ok=True)
    
    async def add_entry(self, entry_type: str, data: Dict) -> str:
        """
        Add entry to ledger
        
        Args:
            entry_type: Type of entry (incident, paradox, guard_rail, etc.)
            data: Entry data
        
        Returns:
            Entry hash
        """
        # Create new entry
        entry = MythGraphEntry(entry_type, data)
        
        # Calculate hash with previous entry
        previous_hash = self.ledger_hash if self.entries else None
        entry_hash = entry.calculate_hash(previous_hash)
        
        # Sign entry
        entry.sign_entry(self.private_key)
        
        # Add to ledger
        self.entries.append(entry)
        self.ledger_hash = entry_hash
        
        # Save to storage
        await self.save_entry(entry)
        
        return entry_hash
    
    async def save_entry(self, entry: MythGraphEntry):
        """Save entry to persistent storage"""
        try:
            # Create filename with timestamp and hash
            filename = f"{entry.timestamp}_{entry.hash[:8]}.json"
            filepath = self.storage_path / filename
            
            # Save entry data
            with open(filepath, 'w') as f:
                json.dump(entry.to_dict(), f, indent=2)
                
        except Exception as e:
            # Log error but don't fail the operation
            print(f"Warning: Failed to save entry to storage: {e}")
    
    async def get_entries(self, entry_type: Optional[str] = None, limit: Optional[int] = None) -> List[Dict]:
        """
        Get entries from ledger
        
        Args:
            entry_type: Filter by entry type
            limit: Maximum number of entries to return
        
        Returns:
            List of entry dictionaries
        """
        filtered_entries = self.entries
        
        if entry_type:
            filtered_entries = [e for e in self.entries if e.entry_type == entry_type]
        
        if limit:
            filtered_entries = filtered_entries[-limit:]
        
        return [entry.to_dict() for entry in filtered_entries]
    
    async def get_entry_by_hash(self, entry_hash: str) -> Optional[Dict]:
        """Get specific entry by hash"""
        for entry in self.entries:
            if entry.hash == entry_hash:
                return entry.to_dict()
        return None
    
    async def verify_ledger_integrity(self) -> Dict:
        """Verify the integrity of the entire ledger"""
        verification_results = {
            "total_entries": len(self.entries),
            "verified_entries": 0,
            "failed_entries": 0,
            "errors": []
        }
        
        previous_hash = None
        
        for i, entry in enumerate(self.entries):
            try:
                # Verify hash calculation
                calculated_hash = entry.calculate_hash(previous_hash)
                if calculated_hash != entry.hash:
                    verification_results["errors"].append({
                        "entry_index": i,
                        "error": "Hash mismatch",
                        "expected": calculated_hash,
                        "actual": entry.hash
                    })
                    verification_results["failed_entries"] += 1
                    continue
                
                # Verify signature
                if not entry.verify_signature(self.public_key):
                    verification_results["errors"].append({
                        "entry_index": i,
                        "error": "Signature verification failed"
                    })
                    verification_results["failed_entries"] += 1
                    continue
                
                # Verify previous hash chain
                if i > 0 and entry.previous_hash != previous_hash:
                    verification_results["errors"].append({
                        "entry_index": i,
                        "error": "Previous hash mismatch",
                        "expected": previous_hash,
                        "actual": entry.previous_hash
                    })
                    verification_results["failed_entries"] += 1
                    continue
                
                verification_results["verified_entries"] += 1
                previous_hash = entry.hash
                
            except Exception as e:
                verification_results["errors"].append({
                    "entry_index": i,
                    "error": f"Verification exception: {str(e)}"
                })
                verification_results["failed_entries"] += 1
        
        verification_results["integrity_score"] = (
            verification_results["verified_entries"] / verification_results["total_entries"]
            if verification_results["total_entries"] > 0 else 0.0
        )
        
        return verification_results
    
    async def export_ledger(self, format: str = "json") -> str:
        """Export ledger in specified format"""
        if format == "json":
            export_data = {
                "ledger_info": {
                    "public_key": self.public_key,
                    "total_entries": len(self.entries),
                    "ledger_hash": self.ledger_hash,
                    "export_timestamp": datetime.utcnow().isoformat()
                },
                "entries": [entry.to_dict() for entry in self.entries]
            }
            
            export_path = self.storage_path / f"ledger_export_{int(time.time())}.json"
            with open(export_path, 'w') as f:
                json.dump(export_data, f, indent=2)
            
            return str(export_path)
        
        elif format == "csv":
            import csv
            
            export_path = self.storage_path / f"ledger_export_{int(time.time())}.csv"
            
            with open(export_path, 'w', newline='') as f:
                writer = csv.writer(f)
                
                # Write header
                writer.writerow([
                    "entry_type", "timestamp", "hash", "signature", "previous_hash", "data"
                ])
                
                # Write entries
                for entry in self.entries:
                    writer.writerow([
                        entry.entry_type,
                        entry.timestamp,
                        entry.hash,
                        entry.signature,
                        entry.previous_hash,
                        json.dumps(entry.data)
                    ])
            
            return str(export_path)
        
        else:
            raise ValueError(f"Unsupported export format: {format}")
    
    async def get_statistics(self) -> Dict:
        """Get ledger statistics"""
        stats = {
            "total_entries": len(self.entries),
            "entry_types": {},
            "timeline": {
                "earliest_entry": None,
                "latest_entry": None,
                "total_duration_hours": 0
            }
        }
        
        # Count entry types
        for entry in self.entries:
            entry_type = entry.entry_type
            if entry_type not in stats["entry_types"]:
                stats["entry_types"][entry_type] = 0
            stats["entry_types"][entry_type] += 1
        
        # Calculate timeline
        if self.entries:
            timestamps = [datetime.fromisoformat(entry.timestamp) for entry in self.entries]
            stats["timeline"]["earliest_entry"] = min(timestamps).isoformat()
            stats["timeline"]["latest_entry"] = max(timestamps).isoformat()
            
            duration = max(timestamps) - min(timestamps)
            stats["timeline"]["total_duration_hours"] = duration.total_seconds() / 3600
        
        return stats
    
    async def cleanup_old_entries(self, max_age_hours: int = 24 * 7) -> int:
        """Remove entries older than specified age"""
        cutoff_time = datetime.utcnow().timestamp() - (max_age_hours * 3600)
        removed_count = 0
        
        # Filter out old entries
        new_entries = []
        for entry in self.entries:
            entry_timestamp = datetime.fromisoformat(entry.timestamp).timestamp()
            if entry_timestamp > cutoff_time:
                new_entries.append(entry)
            else:
                removed_count += 1
        
        # Rebuild ledger with remaining entries
        self.entries = new_entries
        
        # Recalculate hashes
        previous_hash = None
        for entry in self.entries:
            entry.calculate_hash(previous_hash)
            entry.sign_entry(self.private_key)
            previous_hash = entry.hash
        
        if self.entries:
            self.ledger_hash = self.entries[-1].hash
        else:
            self.ledger_hash = "0000000000000000000000000000000000000000000000000000000000000000"
        
        return removed_count
