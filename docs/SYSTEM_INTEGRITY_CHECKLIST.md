# 🛡️ SYSTEM INTEGRITY CHECKLIST - The 12 Project
**Date**: 2025-08-24  
**Purpose**: Prevent future drift incidents and maintain system integrity  
**Status**: ACTIVE - MUST BE FOLLOWED BY ALL AGENTS

## 🚨 CRITICAL PROTECTION ZONES

### **CORE SYSTEM COMPONENTS - NEVER MOVE OR DELETE**
These files are **ABSOLUTELY PROTECTED** and must never be moved to cleanup:

#### **Essential Engine Files**
- ✅ `agents/kai_enhanced_ensemble.py` - MAIN ENSEMBLE ENGINE
- ✅ `neuromem.rs` - Core Rust neural memory engine
- ✅ `ai_scheduler.rs` - Hardware-aware AI task scheduler
- ✅ `setup_models.py` - Model setup and configuration tool
- ✅ `test_kai_ensemble_comprehensive.py` - Main testing framework

#### **Core System Directories**
- ✅ `mythgraph/` - Knowledge graph and pattern storage
- ✅ `models/` - Model files storage
- ✅ `docs/` - Project documentation
- ✅ `consensus/` - Consensus generation components
- ✅ `emotional/` - Emotional intelligence components

#### **Configuration and Build Files**
- ✅ `Cargo.toml` - Rust project configuration
- ✅ `requirements.txt` - Python dependencies
- ✅ `README.md` - Main project documentation
- ✅ `AGENT_READ_FIRST.md` - Agent instructions

## 🔒 CLEANUP RESTRICTIONS

### **What CANNOT Be Moved to Cleanup**
- ❌ **Core system files** (agents, engines, schedulers)
- ❌ **Essential configuration files** (Cargo.toml, requirements.txt)
- ❌ **Main documentation** (README.md, AGENT_READ_FIRST.md)
- ❌ **System directories** (mythgraph, models, consensus)
- ❌ **Testing frameworks** (test files, validation scripts)
- ❌ **Build artifacts** (compiled libraries, binaries)

### **What CAN Be Moved to Cleanup**
- ✅ **Old agent versions** (previous iterations)
- ✅ **Legacy test systems** (outdated testing frameworks)
- ✅ **Separate projects** (unrelated codebases)
- ✅ **Research backups** (valuable but non-essential tools)
- ✅ **Documentation archives** (old versions, drafts)
- ✅ **Temporary files** (logs, cache, temp outputs)

## 📋 PRE-CLEANUP VALIDATION CHECKLIST

### **Before Moving ANY File to Cleanup**
1. **Verify file purpose** - Is this essential to core system operation?
2. **Check dependencies** - Does any core component depend on this file?
3. **Confirm non-essential status** - Is this truly optional/legacy?
4. **Document the move** - Log what was moved and why
5. **Test system integrity** - Ensure no functionality was broken

### **Required Validation Steps**
```bash
# 1. Check if file is imported by core components
grep -r "import.*filename" agents/ mythgraph/ consensus/

# 2. Verify system still works after move
python test_kai_ensemble_comprehensive.py

# 3. Check for missing dependencies
python -c "import filename; print('File accessible')"
```

## 🚨 DRIFT DETECTION PROTOCOLS

### **Automatic Detection**
- **File monitoring** - Track all file movements
- **Dependency checking** - Verify imports still work
- **System testing** - Run tests after any file changes
- **Logging** - Document all file operations

### **Manual Verification**
- **Daily integrity checks** - Verify core files exist
- **Weekly dependency scans** - Check import statements
- **Monthly system tests** - Full validation runs
- **Quarterly audits** - Comprehensive system review

## 🔧 RECOVERY PROCEDURES

### **If Drift is Detected**
1. **IMMEDIATE HALT** - Stop all operations
2. **Assess damage** - Document what was moved/deleted
3. **Restore components** - Move critical files back
4. **Validate system** - Run comprehensive tests
5. **Document incident** - Update drift recovery logs
6. **Implement prevention** - Strengthen protection measures

### **Recovery Priority Order**
1. **Critical engine files** (ensemble, neuromem, scheduler)
2. **Essential tools** (setup_models, testing frameworks)
3. **Core directories** (mythgraph, models, consensus)
4. **Configuration files** (Cargo.toml, requirements.txt)
5. **Documentation** (README, agent instructions)

## 📚 DOCUMENTATION REQUIREMENTS

### **File Movement Logging**
Every file movement must be logged with:
- **Date and time** of movement
- **File name** and original location
- **Destination** (cleanup subfolder)
- **Reason** for movement
- **Agent** responsible for the move
- **Validation** that system still works

### **Required Log Files**
- `logs/file_movements.md` - All file operations
- `logs/system_integrity.md` - Integrity check results
- `logs/drift_incidents.md` - Any drift events
- `logs/recovery_actions.md` - Recovery procedures

## 🎯 AGENT RESPONSIBILITIES

### **Every Agent Must**
1. **Read this checklist** before any file operations
2. **Validate system integrity** after any changes
3. **Document all actions** in appropriate logs
4. **Report any issues** immediately
5. **Follow recovery procedures** if drift detected

### **Agent Prohibitions**
- ❌ **Moving core files** without validation
- ❌ **Deleting essential components** under any circumstances
- ❌ **Bypassing integrity checks** for any reason
- ❌ **Ignoring drift warnings** or error messages
- ❌ **Working without documentation** of actions

## 🚨 EMERGENCY PROCEDURES

### **Critical Drift Detected**
1. **STOP IMMEDIATELY** - No more file operations
2. **Document the situation** - What's broken, what's missing
3. **Begin recovery** - Restore critical components
4. **Validate functionality** - Test everything works
5. **Update documentation** - Record the incident
6. **Strengthen protection** - Prevent future occurrences

### **Contact Information**
- **Primary Contact**: Project maintainer
- **Backup Contact**: System administrator
- **Emergency Protocol**: Immediate halt and recovery mode

## 📊 INTEGRITY MONITORING

### **Daily Checks**
- [ ] Core files exist and accessible
- [ ] System imports work correctly
- [ ] No new files in cleanup without documentation
- [ ] Logs are up to date

### **Weekly Checks**
- [ ] Full system test run
- [ ] Dependency scan completed
- [ ] File movement log reviewed
- [ ] System performance validated

### **Monthly Checks**
- [ ] Comprehensive system audit
- [ ] All documentation updated
- [ ] Protection measures reviewed
- [ ] Recovery procedures tested

## 🎉 SUCCESS METRICS

### **System Integrity Maintained When**
- ✅ All core components present and functional
- ✅ No essential files in cleanup folders
- ✅ All tests pass (100% success rate)
- ✅ Documentation is accurate and complete
- ✅ No drift incidents recorded
- ✅ Protection measures are active

### **System at Risk When**
- ❌ Any core file is missing or moved
- ❌ Tests fail or show errors
- ❌ Dependencies are broken
- ❌ Documentation is outdated
- ❌ Drift incidents occur
- ❌ Protection measures are bypassed

---

**This checklist is MANDATORY for all agents. Follow it exactly to prevent drift and maintain system integrity.**

**Last Updated**: 2025-08-24  
**Status**: ACTIVE - MUST BE FOLLOWED  
**Next Review**: 2025-09-24 (Monthly)
