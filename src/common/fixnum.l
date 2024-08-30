;;;  SPDX-FileCopyrightText: Copyright 2024 James M. Putnam (putnamjm.design@gmail.com)
;;;  SPDX-License-Identifier: MIT

;;;
;;; fixnum functions
;;;
(mu:intern common "1+"
   (:lambda (n)
     (:if (common:fixnump n)
          (mu:add n 1)
          (common:raise n 'common:1+ "not a fixnum"))))

(mu:intern common "1-"
   (:lambda (n)
     (:if (common:fixnump n)
          (mu:sub n 1)
          (common:raise n 'common:1- "not a fixnum"))))

(mu:intern common "truncate"
   (:lambda (n m)
     (:if (common:fixnump n)
          (:if (common:fixnump m)
               `(,(mu:div n m) ,@(mu:sub n (mu:mul m (mu:div n m))))
               (common:raise m 'common:truncate "not a fixnum"))
          (common:raise n 'common:truncate "not a fixnum"))))

;;; round to negative infinity
(mu:intern common "floor"
   (:lambda (n m)
     (:if (common:fixnump n)
          (:if (common:fixnump m)
               `(,(mu:div n m) ,@(mu:sub n (mu:mul m (mu:div n m))))
               (common:raise m 'common:rem "not a fixnum"))
          (common:raise n 'common:floor "not a fixnum"))))

;;; round to positive infinity
(mu:intern common "ceiling"
   (:lambda (n m)
     (:if (common:fixnump n)
          (:if (common:fixnump m)
               `(,(mu:div n m) ,@(mu:sub n (mu:mul m (mu:div n m))))
               (common:raise m 'common:ceiling "not a fixnum"))
          (common:raise n 'common:ceiling "not a fixnum"))))

(mu:intern common "mod"
   (:lambda (n m)
     (:if (common:fixnump n)
          (:if (common:fixnump m)
               (mu:cdr (common:floor m n))
               (common:raise m 'common:mod "not a fixnum"))
          (common:raise n 'common:mod "not a fixnum"))))

(mu:intern common "rem"
   (:lambda (n m)
     (:if (common:fixnump n)
          (:if (common:fixnump m)
               (mu:cdr (common:truncate m n))
               (common:raise m 'common:rem "not a fixnum"))
          (common:raise n 'common:rem "not a fixnum"))))

;;; round to nearest
(mu:intern common "round"
   (:lambda (n m)
     (:if (common:fixnump n)
          (:if (common:fixnump m)
               (mu:sub (mu:div n m) (mu:sub n (mu:mul m (mu:div n m))))
               (common:raise m 'common:round "not a fixnum"))
          (common:raise n 'common:round "not a fixnum"))))
