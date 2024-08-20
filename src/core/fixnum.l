;;;  SPDX-FileCopyrightText: Copyright 2024 James M. Putnam (putnamjm.design@gmail.com)
;;;  SPDX-License-Identifier: MIT

;;;
;;; fixnums
;;;

;;;
;;; functions
;;;
(mu:intern core "1+"
   (:lambda (n)
     (:if (core:fixnump n)
          (mu:sum n 1)
          (core:raise n 'core:1+ "not a fixnum"))))

(mu:intern core "1-"
   (:lambda (n)
     (:if (core:fixnump n)
          (mu:difference n 1)
          (core:raise n 'core:1- "not a fixnum"))))

(mu:intern core "truncate"
   (:lambda (n m)
     (:if (core:fixnump n)
          (:if (core:fixnump m)
               `(,(mu:quotient n m) ,@(mu:difference n (mu:product m (mu:quotient n m))))
               (core:raise m 'core:truncate "not a fixnum"))
          (core:raise n 'core:truncate "not a fixnum"))))

;;; round to negative infinity
(mu:intern core "floor"
   (:lambda (n m)
     (:if (core:fixnump n)
          (:if (core:fixnump m)
               `(,(mu:quotient n m) ,@(mu:difference n (mu:product m (mu:quotient n m))))
               (core:raise m 'core:rem "not a fixnum"))
          (core:raise n 'core:floor "not a fixnum"))))

;;; round to positive infinity
(mu:intern core "ceiling"
   (:lambda (n m)
     (:if (core:fixnump n)
          (:if (core:fixnump m)
               `(,(mu:quotient n m) ,@(mu:difference n (mu:product m (mu:quotient n m))))
               (core:raise m 'core:ceiling "not a fixnum"))
          (core:raise n 'core:ceiling "not a fixnum"))))

(mu:intern core "mod"
   (:lambda (n m)
     (:if (core:fixnump n)
          (:if (core:fixnump m)
               (mu:cdr (core:floor m n))
               (core:raise m 'core:mod "not a fixnum"))
          (core:raise n 'core:mod "not a fixnum"))))

(mu:intern core "rem"
   (:lambda (n m)
     (:if (core:fixnump n)
          (:if (core:fixnump m)
               (mu:cdr (core:truncate m n))
               (core:raise m 'core:rem "not a fixnum"))
          (core:raise n 'core:rem "not a fixnum"))))

;;; round to nearest
(mu:intern core "round"
   (:lambda (n m)
     (:if (core:fixnump n)
          (:if (core:fixnump m)
               (mu:difference (mu:quotient n m) (mu:difference n (mu:product m (mu:quotient n m))))
               (core:raise m 'core:round "not a fixnum"))
          (core:raise n 'core:round "not a fixnum"))))
