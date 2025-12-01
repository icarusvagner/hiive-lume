CREATE TYPE applicant_status AS ENUM (
    'applied',
    'initial_review',
    'shortlisted',
    'phone_interview',
    'exam_assessment',
    'technical_interview',
    'panel_interview',
    'hr_interview',
    'final_interview',
    'reference_check',
    'background_check',
    'salary_discussion',
    'pending_decision',
    'for_job_offer',
    'offer_released',
    'offer_accepted',
    'pre_employment',
    'ready_for_onboarding',
    'hired',

    -- negative outcomes
    'rejected',
    'withdrawn',
    'offer_declined',
    'failed_requirements',

    -- admin workflow
    'on_hold',
    'blacklisted'
);

CREATE TYPE employment_status AS ENUM (
    'full-time',
    'part-time',
    'contract',
    'temporary',
    'seasonal',
    'intern',
    'apprentice',
    'probationary',
    'casual',
    'project-based'
);

CREATE TYPE employee_lifecycle_status AS ENUM (
    'active',
    'probationary',
    'on_leave',
    'suspended',
    'terminated',
    'resigned',
    'retired',
    'end_of_contract',
    'absent_without_leave',
    'inactive'
);

CREATE TYPE attendance_status AS ENUM (
    'present',
    'late',
    'absent',
    'on_leave',
    'half_day',
    'overtime',
    'rest_day',
    'holiday',
    'work_from_home',
    'field_work'
);

CREATE TYPE leave_request_status AS ENUM (
    'pending',
    'approved',
    'denied',
    'cancelled',
    'for_revision',
    'forwarded',
    'escalated'
);

CREATE TYPE session_type AS ENUM (
    'active',
    'inactive'
);

CREATE TYPE user_session_state AS ENUM (
    'active',
    'idle',
    'away',
    'offline',
    'busy',
    'locked'
);

CREATE TYPE performance_review_status AS ENUM (
    'scheduled',
    'in_progress',
    'submitted',
    'under_review',
    'completed',
    'requires_revision',
    'rejected'
);

CREATE TYPE payroll_status AS ENUM (
    'draft',
    'processing',
    'processed',
    'released',
    'paid',
    'cancelled',
    'reconciled'
);

CREATE TYPE disciplinary_action_status AS ENUM (
    'filed',
    'under_investigation',
    'hearing_scheduled',
    'decision_pending',
    'resolved',
    'dismissed'
);

CREATE TYPE contract_status AS ENUM (
    'active',
    'expiring_soon',
    'for_renewal',
    'renewed',
    'non_renewal',
    'expired'
);

