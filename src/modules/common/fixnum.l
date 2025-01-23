;;;  SPDX-FileCopyrightText: Copyright 2024 James M. Putnam (putnamjm.design@gmail.com)
;;;  SPDX-License-Identifier: MIT

;;;
;;; fixnum functions
;;;
(defun 1+ (n)
  (:if (core:fixnump n)
       (mu:add n 1)
       (core:raise n 'common:1+ "not a fixnum")))

(defun 1- (n)
  (:if (core:fixnump n)
       (mu:sub n 1)
       (core:raise n 'common:1- "not a fixnum")))

;;; round to negative infinity
(mu:intern common "truncate"
   (:lambda (n m)
     (:if (core:fixnump n)
          (:if (core:fixnump m)
               `(,(mu:div n m) ,@(mu:sub n (mu:mul m (mu:div n m))))
               (core:raise m 'common:truncate "not a fixnum"))
          (core:raise n 'common:truncate "not a fixnum"))))

;;; round to negative infinity
(mu:intern common "floor"
   (:lambda (n m)
     (:if (core:fixnump n)
          (:if (core:fixnump m)
               `(,(mu:div n m) ,@(mu:sub n (mu:mul m (mu:div n m))))
               (core:raise m 'common:rem "not a fixnum"))
          (core:raise n 'common:floor "not a fixnum"))))

;;; round to positive infinity
(mu:intern common "ceiling"
   (:lambda (n m)
     (:if (core:fixnump n)
          (:if (core:fixnump m)
               `(,(mu:div n m) ,@(mu:sub n (mu:mul m (mu:div n m))))
               (core:raise m 'common:ceiling "not a fixnum"))
          (core:raise n 'common:ceiling "not a fixnum"))))

;;; round to nearest
(mu:intern common "round"
   (:lambda (n m)
     (:if (core:fixnump n)
          (:if (core:fixnump m)
               (mu:sub (mu:div n m) (mu:sub n (mu:mul m (mu:div n m))))
               (core:raise m 'common:round "not a fixnum"))
          (core:raise n 'common:round "not a fixnum"))))

(mu:intern common "mod"
   (:lambda (n m)
     (:if (core:fixnump n)
          (:if (core:fixnump m)
               (mu:cdr (common:floor m n))
               (core:raise m 'common:mod "not a fixnum"))
          (core:raise n 'common:mod "not a fixnum"))))

(mu:intern common "rem"
   (:lambda (n m)
     (:if (core:fixnump n)
          (:if (core:fixnump m)
               (mu:cdr (common:truncate m n))
               (core:raise m 'common:rem "not a fixnum"))
          (core:raise n 'common:rem "not a fixnum"))))
