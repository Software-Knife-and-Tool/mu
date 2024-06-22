;;;  SPDX-FileCopyrightText: Copyright 2017-2022 James M. Putnam (putnamjm.design@gmail.com)
;;;  SPDX-License-Identifier: MIT

(defun describe (obj)
  (flet ((describe-function (fn)
           (errorp-unless functionp fn "describe-function: not a function")
           (let ((desc (core::fn-lambda-desc fn)))
             (format t "   nreq: ~A~%" (core::fn-nreqs fn))
             (format t " lambda: ~A~%" (core::fn-form fn))
             (cond
               ((null desc) (format t "has no lambda extension~%"))
               (t
                (format t "closure: ~A~%" (core::lambda-closure desc))
                (format t "    env: ~A~%" (core::lambda-env desc))
                (format t " macrop: ~A~%" (core::lambda-macrop desc))
                (format t "   reqs: ~A~%" (core::lambda-req-symbols desc))
                (format t "   rest: ~A~%" (core::lambda-rest-symbol desc))))
             fn))
         (describe-symbol (symbol)
           (errorp-unless symbolp symbol "describe-symbol: not a symbol")
           (format t "is a keyword: ~A~%" (keywordp symbol))
           (format t "        name: ~A~%" (symbol-name symbol))
           (format t "          ns: ~A~%" (symbol-ns symbol))
           (if (boundp symbol)
               (format t "  value: ~A~%" (symbol-value symbol))
               (format t "  value: is unbound~%"))
           symbol)
         (describe-vector (vector)
           (errorp-unless vectorp vector "describe-vector: not a vector")
           (format t "  type: ~A~%" (vector-type vector))
           (format t "length: ~A~%" (length vector))
           vector))
    (cond
      ((consp obj) (format t "is a cons: length ~A ~A~%" (length obj) obj))
      ((functionp obj) (format t "is a function: ~A~%" obj) (describe-function obj))
      ((fixnump obj) (format t "is a fixnum: ~A~%" obj))
      ((stringp obj) (format t "is a string byte vector: ~A~%" obj))
      ((symbolp obj) (format t "is a symbol: ~A~%" obj) (describe-symbol obj))
      ((vectorp obj) (format t "is a vector: ~A~%" obj) (escribe-vector obj))
      (t (format t "is undescribed: ~A~%" obj)))))
