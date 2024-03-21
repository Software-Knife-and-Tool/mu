;;; (mu:make-ns :mu-ns)

(define-macro defun (name lambda &rest body)
  (prelude:warn `',name "name")
  `(mu:intern () "name" (:lambda lambda body)))
